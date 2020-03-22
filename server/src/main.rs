mod ffxiv_data;

use std::error::Error;

use actix_web::{App, HttpServer, Result};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Debug).init();
    HttpServer::new(move || App::new().configure(ffxiv_data::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
