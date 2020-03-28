mod context;

use std::collections::BTreeMap;

use actix_web::{error, http::StatusCode, web, HttpResponse, Responder, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json;

use ffxiv_parser::{Ex, ExList, Language};

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

async fn get_ex(context: Context, param: web::Path<GetExParameter>) -> Result<impl Responder> {
    let ex = Ex::new(context.packages.get(&param.version).unwrap(), &param.ex_name).await?;

    let languages = if let Some(language) = param.language {
        if !ex.languages().iter().any(|&x| x == language) {
            return Err(error::ErrorNotFound("No such language"));
        }
        vec![language]
    } else {
        Vec::from(ex.languages())
    };

    let ex_data = languages.iter().map(|&x| (x as u32, ex.all(x).unwrap())).collect::<BTreeMap<_, _>>();
    let ex_json = serde_json::to_string(&ex_data)?;

    // not using `web::Json` because `web::Json` takes ownership and return value of `ex.read_all()` requires same lifetime as `ex`.
    Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").body(ex_json))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.data(CONTEXT.clone())
        .service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{version}/{ex_name}").route(web::get().to(get_ex)))
        .service(web::resource("/parsed/ex/{version}/{language}/{ex_name}").route(web::get().to(get_ex)));
}
