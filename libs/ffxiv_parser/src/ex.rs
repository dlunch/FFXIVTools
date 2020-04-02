mod definition;
mod ex_row;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use exl::ExList;

use std::collections::BTreeMap;
use std::io;

use sqpack_reader::Package;

use ex_row::ExRow;
use exd_map::ExdMap;
use exh::ExHeader;

use crate::Language;

pub struct Ex {
    header: ExHeader,
    data: ExdMap,
}

impl Ex {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let header = ExHeader::new(package, name).await?;
        let data = ExdMap::new(package, name, &header.pages, &header.languages).await?;

        Ok(Self { header, data })
    }

    pub fn languages(&self) -> &[Language] {
        &self.header.languages
    }

    pub fn index(&self, index: u32, language: Language) -> Option<ExRow> {
        let data = self.data.index(index, language)?;

        Some(self.to_row(data))
    }

    pub fn all(&self, language: Language) -> Option<BTreeMap<u32, ExRow>> {
        Some(
            self.data
                .all(language)?
                .map(|(row_id, row_data)| (row_id, self.to_row(row_data)))
                .collect::<BTreeMap<u32, ExRow>>(),
        )
    }

    fn to_row<'a>(&'a self, data: &'a [u8]) -> ExRow<'a> {
        ExRow::new(data, self.header.row_size, &self.header.columns)
    }
}
