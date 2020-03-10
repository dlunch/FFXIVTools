use std::collections::HashMap;
use std::io;

use bytes::Bytes;

use sqpack_reader::Package;

use super::definition::ExhPage;
use super::exd::ExData;
use crate::Language;

pub struct ExdMap {
    data: HashMap<Language, Vec<(ExhPage, ExData)>>,
}

impl ExdMap {
    pub async fn new(package: &dyn Package, name: &str, pages: &[ExhPage], languages: &[Language]) -> io::Result<Self> {
        let mut data = HashMap::with_capacity(languages.len());
        for language in languages {
            let mut page_data = Vec::with_capacity(pages.len());
            for page in pages {
                let exd = ExData::new(package, name, page.start, *language).await?;
                page_data.push((*page, exd));
            }
            data.insert(*language, page_data);
        }

        Ok(Self { data })
    }

    pub fn read_row(&self, index: u32, language: Language) -> Option<Bytes> {
        let items = self.data.get(&language)?;
        let item = items.iter().find(|x| x.0.start <= index && index < x.0.start + x.0.count)?;

        item.1.read_row(index)
    }
}
