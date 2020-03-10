use std::io;

use bytes::Buf;
use num_traits::cast::FromPrimitive;

use sqpack_reader::Package;

use super::definition::{ExhColumnDefinition, ExhHeader, ExhPage};
use crate::Language;

pub struct ExHeader {
    pub columns: Vec<ExhColumnDefinition>,
    pub pages: Vec<ExhPage>,
    pub languages: Vec<Language>,
    pub row_size: u16,
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let mut data = package.read_file(&format!("exd/{}.exh", name)).await?;

        let header = parse!(data, ExhHeader);
        let columns = parse!(data, header.column_count as usize, ExhColumnDefinition);
        let pages = parse!(data, header.page_count as usize, ExhPage);
        let languages = (0..header.language_count as usize)
            .map(|_| Language::from_u64(data.get_u16_le() as u64).unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            columns,
            pages,
            languages,
            row_size: header.row_size,
        })
    }
}
