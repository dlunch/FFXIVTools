use actix_web::{web, HttpResponse, Responder, Result};

use ffxiv_parser::ExList;

use super::context::Context;

async fn get_exl(context: web::Data<Context>) -> Result<impl Responder> {
    let exl = ExList::new(&*context.package).await?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ex"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex").route(web::get().to(get_ex)));
}
