use actix_web::{web, HttpResponse, Result};

use ffxiv_parser::ExList;

use super::context::Context;

async fn get_exl(context: web::Data<Context>) -> Result<HttpResponse> {
    let exl = ExList::new(&*context.package).await?;
    let exl_str = exl.ex_names.join("\n");

    Ok(HttpResponse::Ok().body(exl_str))
}

async fn get_ex() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ex"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex").route(web::get().to(get_ex)));
}
