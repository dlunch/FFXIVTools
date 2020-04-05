use alloc::{format, vec::Vec};

use bytes::Bytes;
use sqpack_reader::{Package, Result};
use util::parse;

use super::definition::{ExhColumnDefinition, ExhHeader, ExhPage};
use crate::Language;

pub struct ExHeader {
    pub row_size: u16,
    pub columns: Vec<ExhColumnDefinition>,
    pub pages: Vec<ExhPage>,
    pub languages: Vec<Language>,
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> Result<Self> {
        let data: Bytes = package.read_file(&format!("exd/{}.exh", name)).await?;

        let header = parse!(data, ExhHeader);
        let columns = parse!(&data[ExhHeader::SIZE..], header.column_count as usize, ExhColumnDefinition);

        let pages_base = ExhHeader::SIZE + header.column_count as usize * ExhColumnDefinition::SIZE;
        let pages = parse!(&data[pages_base..], header.page_count as usize, ExhPage);

        let languages_base = ExhHeader::SIZE + header.column_count as usize * ExhColumnDefinition::SIZE + header.page_count as usize * ExhPage::SIZE;
        let languages = parse!(&data[languages_base..], header.language_count as usize, Language);

        Ok(Self {
            row_size: header.row_size,
            columns,
            pages,
            languages,
        })
    }
}
