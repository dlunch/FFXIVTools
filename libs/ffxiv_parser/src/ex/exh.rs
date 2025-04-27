use alloc::{format, vec::Vec};
use core::mem::size_of;

use sqpack::{Package, Result};
use util::{SliceByteOrderExt, cast};

use super::definition::{ExRowType, ExhColumnDefinition, ExhHeader, ExhPage};
use crate::Language;

pub struct ExHeader {
    pub row_size: u16,
    pub row_type: ExRowType,
    pub columns: Vec<ExhColumnDefinition>,
    pub pages: Vec<ExhPage>,
    pub languages: Vec<Language>,
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> Result<Self> {
        let data = package.read_file(&format!("exd/{name}.exh")).await?;

        let header = cast::<ExhHeader>(&data);

        let columns = (0..header.column_count.get() as usize)
            .map(|x| cast::<ExhColumnDefinition>(&data[size_of::<ExhHeader>() + x * size_of::<ExhColumnDefinition>()..]).clone())
            .collect::<Vec<_>>();

        let pages_base = size_of::<ExhHeader>() + header.column_count.get() as usize * size_of::<ExhColumnDefinition>();
        let pages = (0..header.page_count.get() as usize)
            .map(|x| ExhPage::from_raw(&data[pages_base + x * size_of::<ExhPage>()..]))
            .collect::<Vec<_>>();

        let languages_base = size_of::<ExhHeader>()
            + header.column_count.get() as usize * size_of::<ExhColumnDefinition>()
            + header.page_count.get() as usize * size_of::<ExhPage>();
        let languages = (0..header.language_count.get() as usize)
            .map(|x| Language::from_raw((&data[languages_base + x * size_of::<Language>()..]).to_int_le::<u16>()))
            .collect::<Vec<_>>();

        Ok(Self {
            row_size: header.row_size.get(),
            row_type: ExRowType::from_raw(header.row_type.get()),
            columns,
            pages,
            languages,
        })
    }
}
