use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
use itertools::Itertools;
use log::info;
use serde_json;

use ffxiv_parser::{Ex, ExList};
use sqpack_reader::SqPackReaderFile;

const REGIONS: [&str; 3] = ["kor", "chn", "global"];

struct Context {
    pub package: SqPackReaderFile,
}

impl Context {
    pub fn new() -> Result<Self, io::Error> {
        #[cfg(unix)]
        let path_base = "/mnt/i/FFXIVData/data";
        #[cfg(windows)]
        let path_base = "i:\\FFXIVData\\data";

        let packs = Path::new(path_base)
            .read_dir()?
            .filter_map(Result::ok)
            .map(|x| (x.path(), x.path().file_name().unwrap().to_str().unwrap().to_owned()))
            .map(|(path, file_name)| {
                let mut split = file_name.split('_');
                Some((path, split.next()?.to_owned(), split.next()?.to_owned()))
            })
            .filter_map(|x| x)
            .sorted_by_key(|(_, region, _)| REGIONS.iter().position(|x| x == region))
            .collect::<Vec<_>>();

        info!(
            "mounting {:?}",
            packs
                .iter()
                .map(|(_, region, version)| format!("{}_{}", region, version))
                .collect::<Vec<_>>()
        );

        let paths = packs.into_iter().map(|(path, _, _)| path).collect::<Vec<_>>();
        let package = sqpack_reader::SqPackReaderFile::new(sqpack_reader::FileProviderFile::with_paths(paths))?;

        Ok(Self { package })
    }
}

async fn get_exl(context: web::Data<Context>) -> Result<impl Responder> {
    let exl = ExList::new(&context.package).await?;

    Ok(web::Json(exl.ex_names))
}

async fn get_ex(context: web::Data<Context>, param: web::Path<(String,)>) -> Result<impl Responder> {
    let ex_name = &param.0;
    let ex = Ex::new(&context.package, ex_name).await?;

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
