use std::collections::HashMap;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::info;

use sqpack_reader::{ExtractedFileProviderLocal, SqPackReaderExtractedFile};

const REGIONS: [&str; 3] = ["kor", "chn", "global"];

lazy_static! {
    static ref CONTEXT: Context = Context::new().unwrap();
}

pub struct ContextImpl {
    pub all_package: SqPackReaderExtractedFile,
    pub packages: HashMap<String, SqPackReaderExtractedFile>,
}

impl ContextImpl {
    pub fn new() -> io::Result<Self> {
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
            .sorted_by_key(|(_, region, version)| REGIONS.iter().position(|x| x == region).unwrap() * 1000 + version.parse::<usize>().unwrap())
            .map(|(path, region, version)| (path, format!("{}_{}", region, version)))
            .rev()
            .collect::<Vec<_>>();

        info!("mounting {:?}", packs.iter().map(|(_, key)| key).collect::<Vec<_>>());

        let packages = packs
            .iter()
            .map(|(path, key)| {
                Ok((
                    key.to_owned(),
                    SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_path(&path))?,
                ))
            })
            .collect::<sqpack_reader::Result<HashMap<_, _>>>()
            .map_err(|x| io::Error::new(io::ErrorKind::NotFound, x.to_string()))?;

        let all_paths = packs.into_iter().map(|(path, _)| path).collect::<Vec<_>>();
        let all_package = SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_paths(all_paths))
            .map_err(|x| io::Error::new(io::ErrorKind::NotFound, x.to_string()))?;

        Ok(Self { all_package, packages })
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
