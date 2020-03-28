mod context;

use std::collections::BTreeMap;

use actix_web::{error, web, Responder, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;

use ffxiv_parser::{Ex, ExList, Language};
use sqpack_reader::Package;

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

async fn get_exl(context: Context) -> Result<impl Responder> {
    let exl = ExList::new(&context.all_package).await?;

    Ok(web::Json(exl.ex_names))
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

async fn get_ex(context: Context, param: web::Path<GetExParameter>) -> Result<impl Responder> {
    let package = context.packages.get(&param.version).unwrap();
    let result = ex_to_json(package, param.language, &param.ex_name).await?;

    Ok(web::Json(result))
}

async fn get_ex_bulk(context: Context, param: web::Path<(String, Language, String)>) -> Result<impl Responder> {
    let (version, language, ex_names) = param.into_inner();

    let package = context.packages.get(&version).unwrap();
    let mut result = BTreeMap::new();

    for ex_name in ex_names.split('.') {
        let item = ex_to_json(package, Some(language), &ex_name).await?;

        result.insert(ex_name.to_owned(), item);
    }

    Ok(web::Json(result))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.data(CONTEXT.clone())
        .service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{version}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/{version}/{language}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/bulk/{version}/{language}/{ex_names}").route(web::get().to(get_ex_bulk)));
}
