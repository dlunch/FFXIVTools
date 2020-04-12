use alloc::{format, vec::Vec};
use core::mem::size_of;

use bytes::Bytes;
use sqpack_reader::{Package, Result};
use util::cast;
use zerocopy::LayoutVerified;

use super::definition::{ExRowType, ExhColumnDefinition, ExhHeader, ExhPage};
use crate::Language;

pub struct ExHeader {
    pub row_size: u16,
    pub row_type: ExRowType,
    pub columns: Vec<ExhColumnDefinition>,
    pub pages: Vec<ExhPage>,
    pub languages: Vec<Language>,
}

macro_rules! cast_clone_vec {
    ($data: expr, $count: expr, $type: ty) => {
        (0..$count as usize)
            .map(|x| cast!($data[x * size_of::<$type>()..], $type).clone())
            .collect::<Vec<_>>()
    };
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> Result<Self> {
        let data: Bytes = package.read_file(&format!("exd/{}.exh", name)).await?;

        let header = cast!(data, ExhHeader);
        let columns = cast_clone_vec!(&data[size_of::<ExhHeader>()..], header.column_count.get(), ExhColumnDefinition);
        let pages_base = size_of::<ExhHeader>() + header.column_count.get() as usize * size_of::<ExhColumnDefinition>();
        let pages = (0..header.page_count.get() as usize)
            .map(|x| ExhPage::from(&data[pages_base + x * size_of::<ExhPage>()..]))
            .collect::<Vec<_>>();

        let languages_base = size_of::<ExhHeader>()
            + header.column_count.get() as usize * size_of::<ExhColumnDefinition>()
            + header.page_count.get() as usize * size_of::<ExhPage>();
        let languages = (0..header.language_count.get() as usize)
            .map(|x| Language::from(&data[languages_base + x * size_of::<Language>()..]))
            .collect::<Vec<_>>();

        Ok(Self {
            row_size: header.row_size.get(),
            row_type: ExRowType::from(header.row_type.get()),
            columns,
            pages,
            languages,
        })
    }
}
