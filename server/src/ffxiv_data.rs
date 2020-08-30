mod context;

use std::{collections::BTreeMap, path::PathBuf};

use futures::{
    future,
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use rocket::{
    get,
    response::{content, status},
    routes, Rocket, State,
};
use serde::Serialize;

use ffxiv_parser::{Ex, ExList, ExRowType, Language, Lgb, Lvb};
use sqpack::{Package, SqPackFileHash};
use sqpack_extension::SqPackReaderExtractedFile;

use context::Context;

async fn ex_to_json(package: &dyn Package, language: Option<Language>, ex_name: &str) -> Result<String, status::NotFound<&'static str>> {
    let ex = Ex::new(package, &ex_name).await.map_err(|_| status::NotFound("Not found"))?;

    let languages = if let Some(language) = language {
        if ex.languages()[0] == Language::None {
            vec![Language::None]
        } else {
            if !ex.languages().iter().any(|&x| x == language) {
                return Err(status::NotFound("No such language"));
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

fn find_package<'a>(context: &'a Context, version: &str) -> Result<&'a SqPackReaderExtractedFile, status::NotFound<&'static str>> {
    Ok(context.packages.get(version).ok_or_else(|| status::NotFound("No such package"))?)
}

/// routes

#[get("/parsed/exl/<version>")]
async fn get_exl(context: State<'_, Context>, version: String) -> Result<content::Json<String>, status::NotFound<&'static str>> {
    let package = find_package(&context, &version)?;
    let exl = ExList::new(package).await.map_err(|_| status::NotFound("Not found"))?;

    Ok(content::Json(serde_json::to_string(&exl.ex_names).unwrap()))
}

#[get("/parsed/ex/<version>/<language>/<ex_name>")]
async fn get_ex(
    context: State<'_, Context>,
    version: String,
    language: u16,
    ex_name: String,
) -> Result<content::Json<String>, status::NotFound<&'static str>> {
    let package = find_package(&context, &version)?;
    let result = ex_to_json(package, Some(Language::from_raw(language)), &ex_name).await?;

    Ok(content::Json(result))
}

#[get("/parsed/ex/bulk/<version>/<language>/<ex_names>")]
async fn get_ex_bulk(
    context: State<'_, Context>,
    version: String,
    language: u16,
    ex_names: String,
) -> Result<content::Json<String>, status::NotFound<&'static str>> {
    let language = Language::from_raw(language);

    let package = find_package(&context, &version)?;
    let ex_jsons = ex_names
        .split('.')
        .map(|ex_name| ex_to_json(package, Some(language), &ex_name).map(move |data| Ok::<_, _>((ex_name.to_owned(), data?))))
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
    result.push_str("}");

    Ok(content::Json(result))
}

#[get("/parsed/lvb/<version>/<path..>")]
async fn get_lvb(context: State<'_, Context>, version: String, path: PathBuf) -> Result<content::Json<String>, status::NotFound<&'static str>> {
    let package = find_package(&context, &version)?;

    let lvb = Lvb::new(package, &path.to_str().unwrap())
        .await
        .map_err(|_| status::NotFound("Not found"))?;

    let layers = future::try_join_all(lvb.lgb_paths.into_iter().map(|lgb_path| Lgb::new(package, lgb_path)))
        .await
        .map_err(|_| status::NotFound("Not found"))?
        .into_iter()
        .map(|x| (x.name().to_owned(), x))
        .collect::<BTreeMap<_, _>>();

    #[derive(Serialize)]
    struct JsonLvb {
        layers: BTreeMap<String, Lgb>,
    }

    Ok(content::Json(serde_json::to_string(&JsonLvb { layers }).unwrap()))
}

#[get("/compressed/<version>/<folder_hash>/<file_hash>/<path_hash>")]
async fn get_compressed(
    context: State<'_, Context>,
    version: String,
    folder_hash: u32,
    file_hash: u32,
    path_hash: u32,
) -> Result<Vec<u8>, status::NotFound<&'static str>> {
    let package = find_package(&context, &version)?;

    let result = package
        .read_as_compressed_by_hash(&SqPackFileHash::from_raw_hash(path_hash, folder_hash, file_hash))
        .await
        .map_err(|_| status::NotFound("Not found"))?;

    Ok(result)
}

#[get("/compressed/<version>/bulk/<paths..>", rank = 0)]
async fn get_compressed_bulk(context: State<'_, Context>, version: String, paths: PathBuf) -> Result<Vec<u8>, status::NotFound<&'static str>> {
    let package = find_package(&context, &version)?;

    let hashes = paths
        .to_str()
        .unwrap()
        .split('.')
        .map(|path| {
            let splitted = path
                .split('-')
                .map(|x| u32::from_str_radix(x, 16))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| status::NotFound("Invalid path"))?;
            if splitted.len() != 3 {
                Err(status::NotFound("Invalid path"))
            } else {
                Ok(SqPackFileHash::from_raw_hash(splitted[2], splitted[0], splitted[1]))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    const BULK_ITEM_HEADER_SIZE: usize = (std::mem::size_of::<u32>()) * 4;
    let total_size = hashes
        .iter()
        .map(|hash| {
            package.read_compressed_size_by_hash(&hash).map(|x| match x {
                Some(x) => Ok(x + BULK_ITEM_HEADER_SIZE as u64),
                None => Err(status::NotFound("No such file")),
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

pub async fn config(rocket: Rocket) -> Result<Rocket, Rocket> {
    Ok(rocket
        .mount("/", routes![get_exl, get_ex, get_ex_bulk, get_lvb, get_compressed, get_compressed_bulk])
        .manage(Context::new().unwrap()))
}
