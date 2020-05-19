use alloc::{collections::BTreeMap, vec::Vec};

use futures::{future, FutureExt};

use sqpack_reader::{Package, Result, SqPackReaderError};

use super::definition::ExhPage;
use super::exd::ExData;
use crate::Language;

pub struct ExdMap {
    data: BTreeMap<Language, Vec<(ExhPage, ExData)>>,
}

impl ExdMap {
    pub async fn new(package: &dyn Package, name: &str, pages: &[ExhPage], languages: &[Language]) -> Result<Self> {
        let data = future::try_join_all(languages.iter().map(|&language| {
            future::try_join_all(
                pages
                    .iter()
                    .map(|&page| ExData::new(package, name, page.start, language).map(move |ex_data| Ok::<_, SqPackReaderError>((page, ex_data?)))),
            )
            .map(move |data| Ok::<_, SqPackReaderError>((language, data?)))
        }))
        .await?
        .into_iter()
        .collect::<BTreeMap<_, _>>();

        Ok(Self { data })
    }

    pub fn index(&self, index: u32, language: Language) -> Option<&[u8]> {
        let items = self.data.get(&language)?;
        let (_, ex_data) = items.iter().find(|(page, _)| page.start <= index && index < page.start + page.count)?;

        ex_data.index(index)
    }

    pub fn all(&self, language: Language) -> Option<impl Iterator<Item = (u32, &[u8])>> {
        let items = self.data.get(&language)?;

        Some(items.iter().flat_map(|(_, ex_data)| ex_data.all()))
    }
}
