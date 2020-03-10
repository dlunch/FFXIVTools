use std::io;

use byteorder::{ByteOrder, LittleEndian};
use num_traits::cast::FromPrimitive;

use sqpack_reader::Package;

use super::definition::{ExhColumnDefinition, ExhHeader, ExhPage};
use crate::Language;

pub struct ExHeader {
    pub row_size: u16,
    pub columns: Vec<ExhColumnDefinition>,
    pub pages: Vec<ExhPage>,
    pub languages: Vec<Language>,
}

impl ExHeader {
    pub async fn new(package: &dyn Package, name: &str) -> io::Result<Self> {
        let data = package.read_file(&format!("exd/{}.exh", name)).await?;

        let mut cursor = 0;
        let header = parse!(data, cursor, ExhHeader);
        let columns = parse!(data, cursor, header.column_count as usize, ExhColumnDefinition);
        let pages = parse!(data, cursor, header.page_count as usize, ExhPage);

        let mut languages = Vec::with_capacity(header.language_count as usize);
        for _ in 0..header.language_count as usize {
            let raw = LittleEndian::read_u16(&data[cursor..]);
            cursor += std::mem::size_of::<u16>();

            languages.push(Language::from_u16(raw).unwrap());
        }

        Ok(Self {
            row_size: header.row_size,
            columns,
            pages,
            languages,
        })
    }
}
