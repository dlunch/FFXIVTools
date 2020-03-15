use std::str;

use util::SliceByteOrderExt;

use super::definition::ExhColumnDefinition;

pub enum ExRowType<'a> {
    String(&'a str),
}

pub struct ExRow<'a> {
    data: &'a [u8],
    row_size: u16,
    columns: &'a [ExhColumnDefinition],
}

impl<'a> ExRow<'a> {
    pub fn new(data: &'a [u8], row_size: u16, columns: &'a [ExhColumnDefinition]) -> Self {
        Self { data, row_size, columns }
    }

    pub fn index(&self, index: usize) -> ExRowType {
        let field_type = self.columns[index].field_type;

        match field_type {
            0 => ExRowType::String(self.string(index)),
            _ => panic!(),
        }
    }

    pub fn string(&self, index: usize) -> &str {
        let data_offset = self.columns[index].offset as usize;
        let data = &self.data[data_offset..];
        let str_offset = data.read_u32_be() as usize + self.row_size as usize;

        let bytes = &self.data[str_offset..];
        let end = bytes.iter().position(|&x| x == b'\0').unwrap();

        str::from_utf8(&bytes[..end]).unwrap()
    }
}
