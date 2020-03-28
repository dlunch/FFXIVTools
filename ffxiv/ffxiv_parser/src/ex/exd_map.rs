use std::collections::HashMap;
use std::io;

use futures::future::join_all;
use futures::future::FutureExt;

use sqpack_reader::Package;

use super::definition::ExhPage;
use super::exd::ExData;
use crate::Language;

pub struct ExdMap {
    data: HashMap<Language, Vec<(ExhPage, ExData)>>,
}

impl ExdMap {
    pub async fn new(package: &dyn Package, name: &str, pages: &[ExhPage], languages: &[Language]) -> io::Result<Self> {
        let futures = languages.iter().map(|&language| {
            let futures = pages
                .iter()
                .map(|&page| ExData::new(package, name, page.start, language).map(move |ex_data| Ok::<_, io::Error>((page, ex_data?))));

            join_all(futures).map(move |data| (language, data.into_iter().filter_map(Result::ok).collect::<Vec<_>>()))
        });

        let data = join_all(futures).await.into_iter().collect::<HashMap<_, _>>();

        Ok(Self { data })
    }

    pub fn index(&self, index: u32, language: Language) -> Option<&[u8]> {
        let items = self.data.get(&language)?;
        let item = items.iter().find(|x| x.0.start <= index && index < x.0.start + x.0.count)?;

        item.1.index(index)
    }

    pub fn all(&self, language: Language) -> Option<impl Iterator<Item = (u32, &[u8])>> {
        let items = self.data.get(&language)?;

        Some(items.iter().flat_map(|x| x.1.all()))
    }
}
