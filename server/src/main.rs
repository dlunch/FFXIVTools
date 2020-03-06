use std::error::Error;
use std::path::Path;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;

use ffxiv_parser::ExList;
use sqpack_reader::{FileProviderFile, Package, SqPackReaderFile};

struct Context {
    package: Box<dyn Package>,
}

impl Context {
    async fn new() -> Result<Self, Box<dyn Error>> {

        #[cfg(unix)]
        let provider = FileProviderFile::new(Path::new("/mnt/i/FFXIVData/data/kor_505"));
        let package = Box::new(SqPackReaderFile::new(provider)?);

        Ok(Self { package })
    }
}

#[get("/parsed/exl")]
async fn get_exl(context: web::Data<Context>) -> Result<HttpResponse> {
    let exl = ExList::new(&*context.package).await?;
    let exl_str = exl.ex_names.join("\n");

    Ok(HttpResponse::Ok().body(exl_str))
}

#[get("/parsed/ex")]
async fn get_ex() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ex"))
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let context = Context::new().await?;
    let data = web::Data::new(context);

    HttpServer::new(move || App::new().app_data(data.clone()).service(get_exl).service(get_ex))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
