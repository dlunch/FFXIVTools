use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
use serde_json;

use ffxiv_parser::{Ex, ExList};
use sqpack_reader::Package;

struct Context {
    pub package: Box<dyn Package>,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        #[cfg(unix)]
        let package = Box::new(sqpack_reader::SqPackReaderFile::new(sqpack_reader::FileProviderFile::with_path(
            Path::new("/mnt/i/FFXIVData/data/kor_505"),
        ))?);
        #[cfg(windows)]
        let package = Box::new(sqpack_reader::SqPackReader::new(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack",
        ))?);

        Ok(Self { package })
    }
}

async fn get_exl(context: web::Data<Context>) -> Result<impl Responder> {
    let exl = ExList::new(&*context.package).await?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex(context: web::Data<Context>, param: web::Path<(String,)>) -> Result<impl Responder> {
    let ex_name = &param.0;
    let ex = Ex::new(&*context.package, ex_name).await?;

    let ex_data = ex.languages().iter().map(|&x| (x as u32, ex.all(x).unwrap())).collect::<BTreeMap<_, _>>();

    let ex_json = serde_json::to_string(&ex_data)?;

    // not using `web::Json` because `web::Json` takes ownership and return value of `ex.read_all()` requires same lifetime as `ex`.
    Ok(HttpResponse::build(StatusCode::OK).content_type("application/json").body(ex_json))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let context = Context::new().unwrap();

    cfg.data(context)
        .service(web::resource("/parsed/exl").route(web::get().to(get_exl)))
        .service(web::resource("/parsed/ex/{ex_name}").route(web::get().to(get_ex)));
}
