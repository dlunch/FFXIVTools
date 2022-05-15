mod context;

use std::collections::BTreeMap;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use futures::{
    future,
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use serde::Serialize;

use ffxiv_parser::{Ex, ExList, ExRowType, Language, Lgb, Lvb};
use sqpack::{Package, SqPackFileHash};
use sqpack_extension::SqPackReaderExtractedFile;

use context::Context;

async fn ex_to_json(package: &dyn Package, language: Option<Language>, ex_name: &str) -> Result<String, StatusCode> {
    let ex = Ex::new(package, ex_name).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let languages = if let Some(language) = language {
        if ex.languages()[0] == Language::None {
            vec![Language::None]
        } else {
            if !ex.languages().iter().any(|&x| x == language) {
                return Err(StatusCode::NOT_FOUND);
            }
            vec![language]
        }
    } else {
        Vec::from(ex.languages())
    };

    if ex.row_type() == ExRowType::Single {
        let result = languages
            .into_iter()
            .map(|x| (x as u32, ex.all(x).unwrap().collect::<BTreeMap<_, _>>()))
            .collect::<BTreeMap<_, _>>();
        Ok(serde_json::to_string(&result).unwrap())
    } else {
        let result = languages
            .into_iter()
            .map(|x| {
                (
                    x as u32,
                    ex.all_multi(x)
                        .unwrap()
                        .map(|(k, v)| (k, v.collect::<BTreeMap<_, _>>()))
                        .collect::<BTreeMap<_, _>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        Ok(serde_json::to_string(&result).unwrap())
    }
}

fn find_package<'a>(context: &'a Context, version: &str) -> Result<&'a SqPackReaderExtractedFile, StatusCode> {
    context.packages.get(version).ok_or(StatusCode::NOT_FOUND)
}

/// routes
async fn get_exl(context: Extension<Context>, Path(version): Path<String>) -> Result<Json<String>, StatusCode> {
    let package = find_package(&context, &version)?;
    let exl = ExList::new(package).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::to_string(&exl.ex_names).unwrap()))
}

async fn get_ex(context: Extension<Context>, Path((version, language, ex_name)): Path<(String, u16, String)>) -> Result<Json<String>, StatusCode> {
    let package = find_package(&context, &version)?;
    let result = ex_to_json(package, Some(Language::from_raw(language)), &ex_name).await?;

    Ok(Json(result))
}

async fn get_ex_bulk(
    context: Extension<Context>,
    Path((version, language, ex_names)): Path<(String, u16, String)>,
) -> Result<Json<String>, StatusCode> {
    let language = Language::from_raw(language);

    let package = find_package(&context, &version)?;
    let ex_jsons = ex_names
        .split('.')
        .map(|ex_name| ex_to_json(package, Some(language), ex_name).map(move |data| Ok::<_, StatusCode>((ex_name.to_owned(), data?))))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    // 20 = ex_name + json separator
    let length = ex_jsons.iter().map(|x| x.1.len() + 20).sum();

    let mut result = String::with_capacity(length);
    result.push_str("{\"");
    for (i, (ex_name, ex_json)) in ex_jsons.into_iter().enumerate() {
        if i != 0 {
            result.push_str(",\"");
        }
        result.push_str(&ex_name);
        result.push_str("\":");
        result.push_str(&ex_json);
    }
    result.push('}');

    Ok(Json(result))
}

#[derive(Serialize)]
struct JsonLvb {
    layers: BTreeMap<String, Lgb>,
}

async fn get_lvb(context: Extension<Context>, Path((version, path)): Path<(String, String)>) -> Result<Json<JsonLvb>, StatusCode> {
    let package = find_package(&context, &version)?;

    let lvb = Lvb::new(package, &path).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let layers = future::try_join_all(lvb.lgb_paths.into_iter().map(|lgb_path| Lgb::new(package, lgb_path)))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?
        .into_iter()
        .map(|x| (x.name().to_owned(), x))
        .collect::<BTreeMap<_, _>>();

    Ok(Json(JsonLvb { layers }))
}

async fn get_compressed(
    context: Extension<Context>,
    Path((version, folder_hash, file_hash, path_hash)): Path<(String, u32, u32, u32)>,
) -> Result<Vec<u8>, StatusCode> {
    let package = find_package(&context, &version)?;

    let result = package
        .read_as_compressed_by_hash(&SqPackFileHash::from_raw_hash(path_hash, folder_hash, file_hash))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(result)
}

async fn get_compressed_bulk(context: Extension<Context>, Path((version, paths)): Path<(String, String)>) -> Result<Vec<u8>, StatusCode> {
    let package = find_package(&context, &version)?;

    let hashes = paths
        .split('.')
        .map(|path| {
            let splitted = path
                .split('-')
                .map(|x| u32::from_str_radix(x, 16))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| StatusCode::NOT_FOUND)?;
            if splitted.len() != 3 {
                Err(StatusCode::NOT_FOUND)
            } else {
                Ok(SqPackFileHash::from_raw_hash(splitted[2], splitted[0], splitted[1]))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    const BULK_ITEM_HEADER_SIZE: usize = (std::mem::size_of::<u32>()) * 4;
    let total_size = hashes
        .iter()
        .map(|hash| {
            package.read_compressed_size_by_hash(hash).map(|x| match x {
                Some(x) => Ok(x + BULK_ITEM_HEADER_SIZE as u64),
                None => Err(StatusCode::NOT_FOUND),
            })
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .sum::<u64>();

    let mut result = Vec::with_capacity(total_size as usize);

    let package = find_package(&context, &version).unwrap();
    for hash in hashes {
        let mut data = package.read_as_compressed_by_hash(&hash).await.unwrap();

        let mut header = Vec::with_capacity(BULK_ITEM_HEADER_SIZE);
        header.extend(hash.folder.to_le_bytes().iter());
        header.extend(hash.file.to_le_bytes().iter());
        header.extend(hash.path.to_le_bytes().iter());
        header.extend((data.len() as u32).to_le_bytes().iter());

        result.append(&mut header);
        result.append(&mut data);
    }

    Ok(result)
}

pub fn router() -> Router {
    let context = Context::new().unwrap();

    Router::new()
        .route("/compressed/:version/:folder_hash/:file_hash/:path_hash", get(get_compressed))
        .route("/compressed/:version/bulk/*paths", get(get_compressed_bulk))
        .route("/parsed/exl/:version", get(get_exl))
        .route("/parsed/ex/:version/:language/:ex_name", get(get_ex))
        .route("/parsed/ex/bulk/:version/:language/:ex_names", get(get_ex_bulk))
        .route("/parsed/lvb/:version/*path", get(get_lvb))
        .layer(Extension(context))
}
