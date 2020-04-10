mod definition;
mod ex_row;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use exl::ExList;

use alloc::collections::BTreeMap;

use sqpack_reader::{Package, Result};
use util::parse;

use definition::{ExdData, ExdMultiRowData, ExdMultiRowDataItem};
use ex_row::ExRow;
use exd_map::ExdMap;
use exh::ExHeader;

use crate::Language;

pub use definition::ExRowType;

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

        let data = parse!(self.data.index(index, language)?, ExdData);

        Some(self.to_row(data.data))
    }

    pub fn all(&self, language: Language) -> Option<BTreeMap<u32, ExRow>> {
        debug_assert!(self.header.row_type == ExRowType::Single);

        Some(
            self.data
                .all(language)?
                .map(|(row_id, row_data)| {
                    let data = parse!(row_data, ExdData);
                    (row_id, self.to_row(data.data))
                })
                .collect::<BTreeMap<u32, ExRow>>(),
        )
    }

    pub fn index_multi(&self, index: u32, sub_index: u16, language: Language) -> Option<ExRow> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        let data = ExdMultiRowData::parse(self.data.index(index, language)?).unwrap().1;
        let offset = (sub_index as usize) * (self.header.row_size as usize + core::mem::size_of::<u16>());
        let item = ExdMultiRowDataItem::parse(&data.data[offset..], self.header.row_size as usize).unwrap().1;

        Some(self.to_row(item.data))
    }

    pub fn all_multi(&self, language: Language) -> Option<BTreeMap<u32, BTreeMap<u16, ExRow>>> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        Some(
            self.data
                .all(language)?
                .map(|(row_id, row_data)| {
                    let data = ExdMultiRowData::parse(row_data).unwrap().1;
                    let rows = (0..data.count as usize)
                        .map(|x| {
                            let offset = x * (self.header.row_size as usize + core::mem::size_of::<u16>());
                            let item = ExdMultiRowDataItem::parse(&data.data[offset..], self.header.row_size as usize).unwrap().1;

                            (item.sub_index, self.to_row(item.data))
                        })
                        .collect::<BTreeMap<_, _>>();

                    (row_id, rows)
                })
                .collect::<BTreeMap<u32, BTreeMap<u16, ExRow>>>(),
        )
    }

    fn to_row<'a>(&'a self, data: &'a [u8]) -> ExRow<'a> {
        ExRow::new(data, self.header.row_size, &self.header.columns)
    }
}
