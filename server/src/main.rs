use std::io;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;

struct Context {
}

impl Context {
    async fn new() -> Self {
        Self { }
    }
}

#[get("/parsed/exl")]
async fn get_exl() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("exl"))
}

#[get("/parsed/ex")]
async fn get_ex() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ex"))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let context = Context::new().await;
    let data = web::Data::new(context);

    HttpServer::new(move || App::new().app_data(data.clone()).service(get_exl).service(get_ex))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
