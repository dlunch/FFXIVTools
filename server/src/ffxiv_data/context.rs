use std::io;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::info;

use sqpack_reader::SqPackReaderFile;

const REGIONS: [&str; 3] = ["kor", "chn", "global"];

lazy_static! {
    static ref CONTEXT: Context = Context::new().unwrap();
}

pub struct ContextImpl {
    pub package: SqPackReaderFile,
}

impl ContextImpl {
    pub fn new() -> Result<Self, io::Error> {
        let path_base = "./data";

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

#[derive(Clone)]
pub struct Context {
    context: Arc<ContextImpl>,
}

impl Context {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {
            context: Arc::new(ContextImpl::new()?),
        })
    }
}

impl Deref for Context {
    type Target = ContextImpl;

    fn deref(&self) -> &ContextImpl {
        &self.context
    }
}

impl FromRequest for Context {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(req.app_data::<web::Data<Context>>().unwrap().get_ref().clone())
    }
}
