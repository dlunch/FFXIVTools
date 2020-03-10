mod definition;
mod ex_row;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use exl::ExList;

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
        let data = ExdMap::new(package, name, &header).await?;

        Ok(Self { header, data })
    }

    pub fn languages(&self) -> &[Language] {
        &self.header.languages
    }

    pub fn find_row(&self, index: u32, language: Language) -> Option<ExRow> {
        let data = self.data.read_row(index, language)?;

        Some(ExRow::new(data, self.header.row_size, &self.header.columns))
    }
}
