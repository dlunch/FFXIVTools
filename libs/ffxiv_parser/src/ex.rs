mod definition;
mod ex_row;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use definition::ExRowType;
pub use exl::ExList;

use alloc::collections::BTreeMap;
use core::mem::size_of;

use zerocopy::LayoutVerified;

use sqpack_reader::{Package, Result};
use util::cast;

use definition::{ExdDataHeader, ExdMultiRowDataHeader, ExdMultiRowDataItemHeader};
use ex_row::ExRow;
use exd_map::ExdMap;
use exh::ExHeader;

use crate::Language;

pub struct Ex {
    header: ExHeader,
    data: ExdMap,
}

impl Ex {
    pub async fn new(package: &dyn Package, name: &str) -> Result<Self> {
        let header = ExHeader::new(package, name).await?;
        let data = ExdMap::new(package, name, &header.pages, &header.languages).await?;

        Ok(Self { header, data })
    }

    pub fn languages(&self) -> &[Language] {
        &self.header.languages
    }

    pub fn row_type(&self) -> ExRowType {
        self.header.row_type
    }

    pub fn index(&self, index: u32, language: Language) -> Option<ExRow> {
        debug_assert!(self.header.row_type == ExRowType::Single);

        let raw = self.data.index(index, language)?;
        let row_data = &raw[size_of::<ExdDataHeader>()..];

        Some(self.to_row(row_data))
    }

    pub fn all(&self, language: Language) -> Option<BTreeMap<u32, ExRow>> {
        debug_assert!(self.header.row_type == ExRowType::Single);

        Some(
            self.data
                .all(language)?
                .map(|(row_id, row_data)| {
                    let data = &row_data[size_of::<ExdDataHeader>()..];
                    (row_id, self.to_row(data))
                })
                .collect::<BTreeMap<u32, ExRow>>(),
        )
    }

    pub fn index_multi(&self, index: u32, sub_index: u16, language: Language) -> Option<ExRow> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        let raw = self.data.index(index, language)?;
        let data = &raw[size_of::<ExdMultiRowDataHeader>()..];
        let (_, row) = self.to_multi_row_item(data, sub_index);

        Some(row)
    }

    pub fn all_multi(&self, language: Language) -> Option<BTreeMap<u32, BTreeMap<u16, ExRow>>> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        Some(
            self.data
                .all(language)?
                .map(|(row_id, row_data)| {
                    let header = cast!(row_data, ExdMultiRowDataHeader);
                    let multi_row_data = &row_data[size_of::<ExdMultiRowDataHeader>()..];

                    let rows = (0..header.count.get())
                        .map(|x| self.to_multi_row_item(multi_row_data, x))
                        .collect::<BTreeMap<_, _>>();
                    (row_id, rows)
                })
                .collect::<BTreeMap<u32, BTreeMap<u16, ExRow>>>(),
        )
    }

    fn to_multi_row_item<'a>(&'a self, multi_row_data: &'a [u8], sub_index: u16) -> (u16, ExRow<'a>) {
        let offset = (sub_index as usize) * (self.header.row_size as usize + size_of::<u16>());
        let header = cast!(&multi_row_data[offset..], ExdMultiRowDataItemHeader);
        let row_data = &multi_row_data[offset + size_of::<ExdMultiRowDataItemHeader>()..];

        (header.sub_index.get(), self.to_row(row_data))
    }

    fn to_row<'a>(&'a self, row_data: &'a [u8]) -> ExRow<'a> {
        ExRow::new(row_data, self.header.row_size, &self.header.columns)
    }
}
