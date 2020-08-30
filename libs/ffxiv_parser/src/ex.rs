mod definition;
mod ex_row;
mod exd;
mod exd_map;
mod exh;
mod exl;

pub use definition::ExRowType;
pub use ex_row::ExRow;
pub use exl::ExList;

use core::mem::size_of;

use sqpack::{Package, Result};
use util::cast;

use definition::{ExdDataHeader, ExdMultiRowDataHeader, ExdMultiRowDataItemHeader};
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
        let data = ExdMap::new(package, name, &header.pages, Self::filter_languages(&header.languages)).await?;

        Ok(Self { header, data })
    }

    pub fn languages(&self) -> &[Language] {
        Self::filter_languages(&self.header.languages)
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

    pub fn all(&self, language: Language) -> Option<impl Iterator<Item = (u32, ExRow)>> {
        debug_assert!(self.header.row_type == ExRowType::Single);

        Some(self.data.all(language)?.map(move |(row_id, row_data)| {
            let data = &row_data[size_of::<ExdDataHeader>()..];
            (row_id, self.to_row(data))
        }))
    }

    pub fn index_multi(&self, index: u32, sub_index: u16, language: Language) -> Option<ExRow> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        let raw = self.data.index(index, language)?;
        let data = &raw[size_of::<ExdMultiRowDataHeader>()..];
        let (_, row) = self.to_multi_row_item(data, sub_index);

        Some(row)
    }

    pub fn all_multi(&self, language: Language) -> Option<impl Iterator<Item = (u32, impl Iterator<Item = (u16, ExRow)>)>> {
        debug_assert!(self.header.row_type == ExRowType::Multi);

        Some(self.data.all(language)?.map(move |(row_id, row_data)| {
            let header = cast::<ExdMultiRowDataHeader>(&row_data);
            let multi_row_data = &row_data[size_of::<ExdMultiRowDataHeader>()..];

            let rows = (0..header.count.get()).map(move |x| self.to_multi_row_item(multi_row_data, x));

            (row_id, rows)
        }))
    }

    fn to_multi_row_item<'a>(&'a self, multi_row_data: &'a [u8], sub_index: u16) -> (u16, ExRow<'a>) {
        let offset = (sub_index as usize) * (self.header.row_size as usize + size_of::<u16>());
        let header = cast::<ExdMultiRowDataItemHeader>(&multi_row_data[offset..]);
        let row_data = &multi_row_data[offset + size_of::<ExdMultiRowDataItemHeader>()..];

        (header.sub_index.get(), self.to_row(row_data))
    }

    fn to_row<'a>(&'a self, row_data: &'a [u8]) -> ExRow<'a> {
        ExRow::new(row_data, self.header.row_size, &self.header.columns)
    }

    fn filter_languages(raw_languages: &[Language]) -> &[Language] {
        match raw_languages[0] {
            Language::None => &[Language::None],
            Language::Japanese => &[Language::Japanese, Language::English, Language::Deutsch, Language::French],
            Language::Korean => &[Language::Korean],
            Language::ChineseSimplified => &[Language::ChineseSimplified],
            _ => panic!(),
        }
    }
}
