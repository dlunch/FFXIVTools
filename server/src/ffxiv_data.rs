mod context;

use std::collections::BTreeMap;
use std::io::Cursor;

use anyhow::anyhow;
use axum::{
    extract::{Extension, Path},
    headers::ContentType,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router, TypedHeader,
};
use futures::{
    future,
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use image::RgbaImage;
use serde::Serialize;

use ffxiv_parser::{Ex, ExList, ExRowType, Language, Lgb, Lvb, Tex};
use sqpack::{Package, SqPackFileHash};

use context::Context;

async fn ex_to_json(package: &dyn Package, language: Option<Language>, ex_name: &str) -> anyhow::Result<serde_json::Value> {
    let ex = Ex::new(package, ex_name).await?;

    let languages = if let Some(language) = language {
        if ex.languages()[0] == Language::None {
            vec![Language::None]
        } else {
            if !ex.languages().iter().any(|&x| x == language) {
                return Err(anyhow!("Language not found"));
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
        Ok(serde_json::to_value(&result).unwrap())
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
        Ok(serde_json::to_value(&result).unwrap())
    }
}

/// routes
async fn get_exl(context: Extension<Context>, Path(version): Path<String>) -> Result<Json<Vec<String>>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
    let exl = ExList::new(package).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(exl.ex_names))
}

async fn get_ex_all(context: Extension<Context>, Path((version, ex_name)): Path<(String, String)>) -> Result<Json<serde_json::Value>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
    let result = ex_to_json(package, None, &ex_name).await.map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(result))
}

async fn get_ex(
    context: Extension<Context>,
    Path((version, language, ex_name)): Path<(String, u16, String)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
    let result = ex_to_json(package, Some(Language::from_raw(language)), &ex_name)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(result))
}

async fn get_ex_bulk_all(
    context: Extension<Context>,
    Path((version, ex_names)): Path<(String, String)>,
) -> Result<Json<BTreeMap<String, serde_json::Value>>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
    let ex_jsons = ex_names
        .split('.')
        .map(|ex_name| {
            ex_to_json(package, None, ex_name).map(move |data| Ok::<_, StatusCode>((ex_name.to_owned(), data.map_err(|_| StatusCode::NOT_FOUND)?)))
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<BTreeMap<_, _>>()
        .await?;

    Ok(Json(ex_jsons))
}

async fn get_ex_bulk(
    context: Extension<Context>,
    Path((version, language, ex_names)): Path<(String, u16, String)>,
) -> Result<Json<BTreeMap<String, serde_json::Value>>, StatusCode> {
    let language = Language::from_raw(language);

    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
    let ex_jsons = ex_names
        .split('.')
        .map(|ex_name| {
            ex_to_json(package, Some(language), ex_name)
                .map(move |data| Ok::<_, StatusCode>((ex_name.to_owned(), data.map_err(|_| StatusCode::NOT_FOUND)?)))
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<BTreeMap<_, _>>()
        .await?;

    Ok(Json(ex_jsons))
}

#[derive(Serialize)]
struct JsonLvb {
    layers: BTreeMap<String, Lgb>,
}

async fn get_lvb(context: Extension<Context>, Path((version, path)): Path<(String, String)>) -> Result<Json<JsonLvb>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;

    let lvb = Lvb::new(package, &format!("bg/{}.lvb", &path[1..]))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let layers = future::try_join_all(lvb.lgb_paths.into_iter().map(|lgb_path| Lgb::new(package, lgb_path)))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?
        .into_iter()
        .map(|x| (x.name().to_owned(), x))
        .collect::<BTreeMap<_, _>>();

    Ok(Json(JsonLvb { layers }))
}

async fn get_tex(context: Extension<Context>, Path((version, path)): Path<(String, String)>) -> Result<impl IntoResponse, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;

    let tex = Tex::new(package, format!("{}.tex", &path[1..]))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let data = Vec::with_capacity((tex.width() * tex.height() / 4) as usize);
    let mut writer = Cursor::new(data);

    let image = RgbaImage::from_raw(tex.width() as u32, tex.height() as u32, tex.data_rgba(0)).unwrap();
    image.write_to(&mut writer, image::ImageOutputFormat::Png).unwrap();

    Ok((TypedHeader(ContentType::png()), writer.into_inner()))
}

async fn get_compressed_all(
    context: Extension<Context>,
    Path((folder_hash, file_hash, path_hash)): Path<(u32, u32, u32)>,
) -> Result<Vec<u8>, StatusCode> {
    get_compressed(context, Path(("all".into(), folder_hash, file_hash, path_hash))).await
}

async fn get_compressed(
    context: Extension<Context>,
    Path((version, folder_hash, file_hash, path_hash)): Path<(String, u32, u32, u32)>,
) -> Result<Vec<u8>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;

    let result = package
        .read_as_compressed_by_hash(&SqPackFileHash::from_raw_hash(path_hash, folder_hash, file_hash))
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(result)
}

async fn get_compressed_bulk_all(context: Extension<Context>, Path(paths): Path<String>) -> Result<Vec<u8>, StatusCode> {
    get_compressed_bulk(context, Path(("all".into(), paths))).await
}

async fn get_compressed_bulk(context: Extension<Context>, Path((version, paths)): Path<(String, String)>) -> Result<Vec<u8>, StatusCode> {
    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;

    let hashes = paths[1..]
        .split('.')
        .map(|path| {
            let splitted = path
                .split('-')
                .map(|x| u32::from_str_radix(x, 16))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            if splitted.len() != 3 {
                Err(StatusCode::BAD_REQUEST)
            } else {
                Ok(SqPackFileHash::from_raw_hash(splitted[2], splitted[0], splitted[1]))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    const BULK_ITEM_HEADER_SIZE: usize = (std::mem::size_of::<u32>()) * 4;
    let total_size = hashes
        .iter()
        .map(|hash| {
            package
                .read_compressed_size_by_hash(hash)
                .map(|x| x.map(|x| x + BULK_ITEM_HEADER_SIZE as u64).ok_or(StatusCode::NOT_FOUND))
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .sum::<u64>();

    let mut result = Vec::with_capacity(total_size as usize);

    let package = context.packages.get(&version).ok_or(StatusCode::NOT_FOUND)?;
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
        // intentionally named as a, b, c, d to workaround https://github.com/ibraheemdev/matchit/issues/13
        .route("/compressed/:a/bulk/*paths", get(get_compressed_bulk))
        .route("/compressed/bulk/*paths", get(get_compressed_bulk_all))
        .route("/compressed/:a/:b/:c/:d", get(get_compressed))
        .route("/compressed/:a/:b/:c", get(get_compressed_all))
        .route("/parsed/exl/:version", get(get_exl))
        .route("/parsed/ex/:a/:b/:c", get(get_ex))
        .route("/parsed/ex/:a/:b", get(get_ex_all))
        .route("/parsed/ex/bulk/:a/:b/:c", get(get_ex_bulk))
        .route("/parsed/ex/bulk/:a/:b", get(get_ex_bulk_all))
        .route("/parsed/lvb/:version/*path", get(get_lvb))
        .route("/parsed/tex/:version/*path", get(get_tex))
        .layer(Extension(context))
}
