mod ffxiv_data;

use std::error::Error;

use actix_web::{
    dev::Service,
    http::{header, HeaderValue},
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use futures::FutureExt;

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
        .set(header::CacheControl(vec![header::CacheDirective::MaxAge(31_536_000)]))
        .body(response)
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Debug).init();
    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                const ALLOWD_ORIGINS: [&str; 2] = ["https://ffxiv-dev.dlunch.net", "http://localhost:8080"];

                let allow_origin;
                if let Some(origin) = req.headers().get(header::ORIGIN) {
                    if ALLOWD_ORIGINS.iter().any(|x| x == origin) {
                        allow_origin = origin.to_owned();
                    } else {
                        allow_origin = HeaderValue::from_static("https://ffxiv.dlunch.net");
                    };
                } else {
                    allow_origin = HeaderValue::from_static("https://ffxiv.dlunch.net");
                }

                srv.call(req).map(|res| {
                    let mut res = res?;

                    res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, allow_origin);
                    res.headers_mut()
                        .insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET"));
                    res.headers_mut()
                        .insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"));
                    res.headers_mut()
                        .insert(header::VARY, HeaderValue::from_static("Origin, Accept-Encoding"));

                    Ok(res)
                })
            })
            .configure(ffxiv_data::config)
            .service(web::resource("/probe").route(web::get().to(probe)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
