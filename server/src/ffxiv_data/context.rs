use std::collections::HashMap;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use itertools::Itertools;
use log::info;

use sqpack_reader::{ExtractedFileProviderLocal, SqPackReaderExtractedFile};

const REGIONS: [&str; 3] = ["kor", "chn", "global"];

pub struct ContextImpl {
    pub packages: HashMap<String, SqPackReaderExtractedFile>,
}

impl ContextImpl {
    pub fn new() -> io::Result<Self> {
        let path_base = "./data";

        let packs = Path::new(path_base)
            .read_dir()?
            .filter_map(Result::ok)
            .filter_map(|x| Some((x.path(), x.path().file_name()?.to_str()?.to_owned())))
            .map(|(path, file_name)| {
                let mut split = file_name.split('_');
                Some((path, split.next()?.to_owned(), split.next()?.parse::<usize>().unwrap()))
            })
            .filter_map(|x| x)
            .sorted_by_key(|(_, region, version)| REGIONS.iter().position(|x| x == region).unwrap() * 1000 + version)
            .map(|(path, region, version)| (path, format!("{}_{}", region, version)))
            .rev()
            .collect::<Vec<_>>();

        info!("mounting {:?}", packs.iter().map(|(_, key)| key).collect::<Vec<_>>());

        let mut packages = packs
            .iter()
            .map(|(path, key)| {
                (
                    key.to_owned(),
                    SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_path(&path)),
                )
            })
            .collect::<HashMap<_, _>>();

        let all_paths = packs.into_iter().map(|(path, _)| path).collect::<Vec<_>>();
        let all_package = SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_paths(all_paths));
        packages.insert("all".to_owned(), all_package);

        Ok(Self { packages })
    }
}

#[derive(Clone)]
pub struct Context {
    context: Arc<ContextImpl>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

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
