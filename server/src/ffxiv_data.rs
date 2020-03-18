use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
use serde_json;

use ffxiv_parser::{Ex, ExList, Language};

use super::context::Context;

async fn get_exl(context: web::Data<Context>) -> Result<impl Responder> {
    let exl = ExList::new(&*context.package).await?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex(context: web::Data<Context>, param: web::Path<(String,)>) -> Result<impl Responder> {
    let ex_name = &param.0;
    let ex = Ex::new(&*context.package, ex_name).await?;

    let ex_json = serde_json::to_string(&ex.read_all(Language::None).unwrap())?;

    Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").body(ex_json))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{ex_name}").route(web::get().to(get_ex)));
}
