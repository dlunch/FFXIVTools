mod context;

use std::collections::BTreeMap;
use std::io;

use actix_web::{error, web, HttpResponse, Responder, Result};
use bytes::{BufMut, Bytes, BytesMut};
use futures::{future, future::FutureExt};
use genawaiter::{rc::gen, yield_};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;

use ffxiv_parser::{Ex, ExList, Language};
use sqpack_reader::{Package, SqPackFileHash};

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

async fn ex_to_json(package: &dyn Package, language: Option<Language>, ex_name: &str) -> Result<serde_json::Value> {
    let ex = Ex::new(package, &ex_name).await?;

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

    let result = languages.iter().map(|&x| (x as u32, ex.all(x).unwrap())).collect::<BTreeMap<_, _>>();

    Ok(serde_json::to_value(result)?)
}

fn find_package<'a>(context: &'a Context, version: &str) -> Result<&'a impl Package> {
    Ok(context
        .packages
        .get(version)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No such package"))?)
}

/// routes

async fn get_exl(context: Context) -> Result<impl Responder> {
    let exl = ExList::new(&context.all_package).await?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex(context: Context, param: web::Path<GetExParameter>) -> Result<impl Responder> {
    let package = find_package(&context, &param.version)?;
    let result = ex_to_json(package, param.language, &param.ex_name).await?;

    Ok(web::Json(result))
}

async fn get_ex_bulk(context: Context, param: web::Path<(String, Language, String)>) -> Result<impl Responder> {
    let (version, language, ex_names) = param.into_inner();

    let package = find_package(&context, &version)?;
    let mut result = BTreeMap::new();

    for ex_name in ex_names.split('.') {
        let item = ex_to_json(package, Some(language), &ex_name).await?;

        result.insert(ex_name.to_owned(), item);
    }

    Ok(web::Json(result))
}

async fn get_compressed(context: Context, param: web::Path<(u32, u32, u32)>) -> Result<impl Responder> {
    let (folder_hash, file_hash, path_hash) = param.into_inner();

    let result = context
        .all_package
        .read_as_compressed_by_hash(&SqPackFileHash::from_raw_hash(path_hash, folder_hash, file_hash))
        .await?;

    Ok(HttpResponse::Ok().content_type("application/octet-stream").body(result))
}

async fn get_compressed_bulk(context: Context, param: web::Path<(String,)>) -> Result<impl Responder> {
    const BULK_ITEM_HEADER_SIZE: u64 = (std::mem::size_of::<u32>() as u64) * 4;

    let paths = param.0.split('.').collect::<Vec<_>>();
    let mut hashes = Vec::with_capacity(paths.len());
    for path in paths {
        let splitted = path
            .split('-')
            .map(|x| u32::from_str_radix(x, 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| error::ErrorBadRequest("Invalid path"))?;
        if splitted.len() < 3 {
            return Err(error::ErrorBadRequest("Invalid path"));
        }
        hashes.push(SqPackFileHash::from_raw_hash(splitted[2], splitted[0], splitted[1]));
    }

    let total_size = future::join_all(hashes.iter().map(|hash| {
        context.all_package.read_compressed_size_by_hash(&hash).map(|x| match x {
            Some(x) => Ok(x + BULK_ITEM_HEADER_SIZE),
            None => Err(error::ErrorNotFound("No such file")),
        })
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .sum();

    let stream = gen!({
        for hash in hashes {
            let data = context.all_package.read_as_compressed_by_hash(&hash).await.unwrap();

            let mut header = BytesMut::with_capacity(BULK_ITEM_HEADER_SIZE as usize);
            header.put_u32_le(hash.folder);
            header.put_u32_le(hash.file);
            header.put_u32_le(hash.path);
            header.put_u32_le(data.len() as u32);

            yield_!(Result::<Bytes>::Ok(header.freeze()));
            yield_!(Result::<Bytes>::Ok(data));
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .content_length(total_size)
        .streaming(stream))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.data(CONTEXT.clone())
        .service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{version}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/{version}/{language}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/bulk/{version}/{language}/{ex_names}").route(web::get().to(get_ex_bulk)))
        .service(web::resource("/compressed/{folder_hash}/{file_hash}/{full_hash}").route(web::get().to(get_compressed)))
        .service(web::resource("/compressed/bulk/{paths}").route(web::get().to(get_compressed_bulk)));
}
