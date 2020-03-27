mod ffxiv_data;

use std::error::Error;

use actix_web::{
    http::header::{CacheControl, CacheDirective},
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

fn probe(req: HttpRequest) -> HttpResponse {
    let ray = req.headers().get("CF_RAY");
    let ipcountry = req.headers().get("CF_IPCOUNTRY");

    let mut enable_cf = true;

    if let (Some(ray), Some(ipcountry)) = (ray, ipcountry) {
        let dc = ray.to_str().unwrap().split('-').nth(1).unwrap();

        if ipcountry == "KR" && dc != "ICN" {
            enable_cf = false;
        }
    }

    let response = if enable_cf {
        "https://ffxiv-data.dlunch.net"
    } else {
        "https://ffxiv-data3.dlunch.net"
    };

    HttpResponse::Ok()
        .set(CacheControl(vec![CacheDirective::MaxAge(31536000)]))
        .body(response)
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Debug).init();
    HttpServer::new(move || {
        App::new()
            .configure(ffxiv_data::config)
            .service(web::resource("/probe").route(web::get().to(probe)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
