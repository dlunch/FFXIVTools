mod context;
mod ffxiv_data;

use std::error::Error;

use actix_web::{web, App, HttpServer, Result};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let context = context::Context::new().await?;
    let data = web::Data::new(context);

    HttpServer::new(move || App::new().app_data(data.clone()).configure(ffxiv_data::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
