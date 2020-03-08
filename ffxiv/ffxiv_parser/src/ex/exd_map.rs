use std::collections::HashMap;
use std::io;

use sqpack_reader::Package;

use super::definition::ExhPageHeader;
use super::exd::ExData;
use super::exh::ExHeader;
use crate::Language;

#[allow(dead_code)] // WIP
pub struct ExdMap {
    data: HashMap<Language, Vec<(ExhPageHeader, ExData)>>,
}

impl ExdMap {
    pub async fn new(package: &dyn Package, name: &str, header: &ExHeader) -> io::Result<Self> {
        let mut data = HashMap::with_capacity(header.languages.len());
        for language in &header.languages {
            let mut page_data = Vec::with_capacity(header.pages.len());
            for page in &header.pages {
                let exd = ExData::new(package, name, page.start, *language).await?;
                page_data.push((*page, exd));
            }
            data.insert(*language, page_data);
        }

        Ok(Self { data })
    }
}
