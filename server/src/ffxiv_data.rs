mod context;

use std::collections::BTreeMap;

use actix_web::{error, web, Error, HttpResponse, Responder, Result};
use bytes::Bytes;
use futures::{
    future,
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use genawaiter::{rc::gen, yield_};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use ffxiv_parser::{Ex, ExList, ExRowType, Language, Lgb, Lvb};
use sqpack_reader::{Package, SqPackFileHash, SqPackReaderExtractedFile};

use context::Context;

#[derive(Deserialize)]
struct GetExParameter {
    version: String,
    ex_name: String,
    language: Option<Language>,
}

lazy_static! {
    static ref CONTEXT: Context = Context::new().unwrap();
}

async fn ex_to_json(package: &dyn Package, language: Option<Language>, ex_name: &str) -> Result<String> {
    let ex = Ex::new(package, &ex_name).await.map_err(|_| error::ErrorNotFound("Not found"))?;

    let languages = if let Some(language) = language {
        if ex.languages()[0] == Language::None {
            vec![Language::None]
        } else {
            if !ex.languages().iter().any(|&x| x == language) {
                return Err(error::ErrorNotFound("No such language"));
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
        Ok(serde_json::to_string(&result)?)
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
        Ok(serde_json::to_string(&result)?)
    }
}

fn find_package<'a>(context: &'a Context, version: &str) -> Result<&'a SqPackReaderExtractedFile> {
    Ok(context.packages.get(version).ok_or_else(|| error::ErrorNotFound("No such package"))?)
}

/// routes

async fn get_exl(context: Context, param: web::Path<(String,)>) -> Result<impl Responder> {
    let (version,) = param.into_inner();

    let package = find_package(&context, &version)?;
    let exl = ExList::new(package).await.map_err(|_| error::ErrorNotFound("Not found"))?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex(context: Context, param: web::Path<GetExParameter>) -> Result<impl Responder> {
    let package = find_package(&context, &param.version)?;
    let result = ex_to_json(package, param.language, &param.ex_name).await?;

    Ok(HttpResponse::Ok().content_type("application/json").body(result))
}

async fn get_ex_bulk(context: Context, param: web::Path<(String, Language, String)>) -> Result<impl Responder> {
    let (version, language, ex_names) = param.into_inner();

    let package = find_package(&context, &version)?;
    let results = ex_names
        .split('.')
        .map(|ex_name| ex_to_json(package, Some(language), &ex_name).map(move |data| Ok::<_, Error>((ex_name.to_owned(), data?))))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    let stream = gen!({
        yield_!(Result::<Bytes>::Ok(Bytes::from_static(b"{\"")));
        for (i, (ex_name, result)) in results.into_iter().enumerate() {
            if i != 0 {
                yield_!(Result::<Bytes>::Ok(Bytes::from_static(b",\"")));
            }
            yield_!(Result::<Bytes>::Ok(Bytes::from(ex_name)));
            yield_!(Result::<Bytes>::Ok(Bytes::from_static(b"\":")));
            yield_!(Result::<Bytes>::Ok(Bytes::from(result)));
        }
        yield_!(Result::<Bytes>::Ok(Bytes::from_static(b"}")));
    });

    Ok(HttpResponse::Ok().content_type("application/json").streaming(stream))
}

async fn get_compressed(context: Context, param: web::Path<(String, u32, u32, u32)>) -> Result<impl Responder> {
    let (version, folder_hash, file_hash, path_hash) = param.into_inner();
    let package = find_package(&context, &version)?;

    let result = package
        .read_as_compressed_by_hash(&SqPackFileHash::from_raw_hash(path_hash, folder_hash, file_hash))
        .await
        .map_err(|_| error::ErrorNotFound("Not found"))?;

    Ok(HttpResponse::Ok().content_type("application/octet-stream").body(result))
}

async fn get_compressed_bulk(context: Context, param: web::Path<(String, String)>) -> Result<impl Responder> {
    let (version, paths) = param.into_inner();
    let package = find_package(&context, &version)?;

    let hashes = paths
        .split('.')
        .map(|path| {
            let splitted = path
                .split('-')
                .map(|x| u32::from_str_radix(x, 16))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| error::ErrorBadRequest("Invalid path"))?;
            if splitted.len() != 3 {
                Err(error::ErrorBadRequest("Invalid path"))
            } else {
                Ok(SqPackFileHash::from_raw_hash(splitted[2], splitted[0], splitted[1]))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    const BULK_ITEM_HEADER_SIZE: usize = (std::mem::size_of::<u32>()) * 4;
    let total_size = hashes
        .iter()
        .map(|hash| {
            package.read_compressed_size_by_hash(&hash).map(|x| match x {
                Some(x) => Ok(x + BULK_ITEM_HEADER_SIZE as u64),
                None => Err(error::ErrorNotFound("No such file")),
            })
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .sum::<u64>();

    let stream = gen!({
        let package = find_package(&context, &version).unwrap();
        for hash in hashes {
            let data = package.read_as_compressed_by_hash(&hash).await.unwrap();

            let mut header = Vec::with_capacity(BULK_ITEM_HEADER_SIZE);
            header.extend(hash.folder.to_le_bytes().iter());
            header.extend(hash.file.to_le_bytes().iter());
            header.extend(hash.path.to_le_bytes().iter());
            header.extend((data.len() as u32).to_le_bytes().iter());

            yield_!(Result::<Bytes>::Ok(header.into()));
            yield_!(Result::<Bytes>::Ok(Bytes::from(data)));
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .content_length(total_size)
        .streaming(stream))
}

async fn get_lvb(context: Context, param: web::Path<(String, String)>) -> Result<impl Responder> {
    let (version, path) = param.into_inner();
    let package = find_package(&context, &version)?;

    let lvb = Lvb::new(package, &path).await.map_err(|_| error::ErrorNotFound("Not found"))?;

    let layers = future::try_join_all(lvb.lgb_paths.into_iter().map(|lgb_path| Lgb::new(package, lgb_path)))
        .await
        .map_err(|_| error::ErrorNotFound("Not found"))?
        .into_iter()
        .map(|x| (x.name().to_owned(), x))
        .collect::<BTreeMap<_, _>>();

    #[derive(Serialize)]
    struct JsonLvb {
        layers: BTreeMap<String, Lgb>,
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&JsonLvb { layers })?))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.data(CONTEXT.clone())
        .service(web::resource("/parsed/exl/{version}").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{version}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/{version}/{language}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/bulk/{version}/{language}/{ex_names}").route(web::get().to(get_ex_bulk)))
        .service(web::resource("/parsed/lvb/{version}/{path:.*}").route(web::get().to(get_lvb)))
        .service(web::resource("/compressed/{version}/{folder_hash}/{file_hash}/{full_hash}").route(web::get().to(get_compressed)))
        .service(web::resource("/compressed/{version}/bulk/{paths}").route(web::get().to(get_compressed_bulk)));
}
