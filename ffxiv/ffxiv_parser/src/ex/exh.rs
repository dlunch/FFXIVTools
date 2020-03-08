use std::io;

use bytes::Buf;
use num_traits::cast::FromPrimitive;

use sqpack_reader::Package;

use super::definition::{ExhColumnHeader, ExhHeader, ExhPageHeader};
use crate::Language;

#[allow(dead_code)] // WIP
pub struct ExHeader {
    pub columns: Vec<ExhColumnHeader>,
    pub pages: Vec<ExhPageHeader>,
    pub languages: Vec<Language>,
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let mut data = package.read_file(&format!("exd/{}.exh", name)).await?;

        let header = parse!(data, ExhHeader);
        let columns = parse!(data, header.column_count as usize, ExhColumnHeader);
        let pages = parse!(data, header.page_count as usize, ExhPageHeader);
        let languages = (0..header.language_count as usize)
            .map(|_| Language::from_u16(data.get_u16()).unwrap())
            .collect::<Vec<_>>();

        Ok(Self { columns, pages, languages })
    }
}
