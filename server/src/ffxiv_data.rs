use std::collections::BTreeMap;
use std::error::Error;
use std::path::PathBuf;

use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
use lazy_static::lazy_static;
use serde_json;

use ffxiv_parser::{Ex, ExList};
use sqpack_reader::SqPackReaderFile;

#[rustfmt::skip]
lazy_static! {
    static ref VERSIONS: Vec<(&'static str, Vec<&'static str>)> = vec![
        (
            "kor",
            vec![
                "320", "330", "335", "338", "340", "345", "350", "355", "357", "400", "401", "405", "406", "410", "411", "415", "420", "425", "430",
                "431", "435", "436", "440", "445", "450", "455", "456", "458", "500", "501", "505",
            ],
        ),
        (
            "chn",
            vec![
                "340", "350", "400", "410", "415", "420", "430", "440", "450", "500", "511"
            ]
        ),
        (
            "global",
            vec![
                "356", "400_bench", "400", "401", "405", "406", "410", "411", "415", "420", "425", "430", "436", "440", "445", "450", "455", "456",
                "500", "505", "510", "511", "515", "520", "521",
            ],
        ),
    ];
}

struct Context {
    pub package: SqPackReaderFile,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        #[cfg(unix)]
        let path_base = "/mnt/i/FFXIVData/data";
        #[cfg(windows)]
        let path_base = "i:\\FFXIVData\\data";

        let version = VERSIONS
            .iter()
            .flat_map(|x| x.1.iter().map(move |y| format!("{}/{}_{}", path_base, x.0, y)))
            .map(PathBuf::from)
            .collect::<Vec<_>>();

        let package = sqpack_reader::SqPackReaderFile::new(sqpack_reader::FileProviderFile::with_paths(version))?;

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
