use std::str;

use util::SliceByteOrderExt;

use super::definition::{ExFieldType, ExhColumnDefinition};

pub enum ExRowItem<'a> {
    String(&'a str),
    Bool(bool),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Float(f32),
    Quad((u16, u16, u16, u16)),
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

    pub fn index(&self, index: usize) -> ExRowItem {
        match self.columns[index].field_type {
            ExFieldType::String => ExRowItem::String(self.string(index)),
            ExFieldType::Bool => ExRowItem::Bool(self.bool(index)),
            ExFieldType::Int8 => ExRowItem::Int8(self.int8(index)),
            ExFieldType::UInt8 => ExRowItem::UInt8(self.uint8(index)),
            ExFieldType::Int16 => ExRowItem::Int16(self.int16(index)),
            ExFieldType::UInt16 => ExRowItem::UInt16(self.uint16(index)),
            ExFieldType::Int32 => ExRowItem::Int32(self.int32(index)),
            ExFieldType::UInt32 => ExRowItem::UInt32(self.uint32(index)),
            ExFieldType::PackedBool => ExRowItem::Bool(self.bool(index)),
            ExFieldType::Float => ExRowItem::Float(self.float(index)),
            ExFieldType::Quad => ExRowItem::Quad(self.quad(index)),
        }
    }

    pub fn string(&self, index: usize) -> &str {
        debug_assert!(self.columns[index].field_type == ExFieldType::String);

        let data = self.data_slice(index);
        let str_offset = data.read_int_be::<u32>() as usize + self.row_size as usize;

        let bytes = &self.data[str_offset..];
        let end = bytes.iter().position(|&x| x == b'\0').unwrap();

        str::from_utf8(&bytes[..end]).unwrap()
    }

    pub fn bool(&self, index: usize) -> bool {
        let packed_bool_offset = ExFieldType::PackedBool as u16;
        let field_type_value = self.columns[index].field_type as u16;

        debug_assert!(self.columns[index].field_type == ExFieldType::Bool || field_type_value >= packed_bool_offset);

        let data;
        if field_type_value >= packed_bool_offset {
            // packed bool
            let packed_data = self.data_slice(index).read_int_le::<u32>();
            let index = field_type_value - packed_bool_offset;
            data = (packed_data & (1 << index)) as u8;
        } else {
            data = self.data_slice(index)[0];
        };

        match data {
            0 => false,
            1 => true,
            _ => panic!(),
        }
    }

    pub fn int8(&self, index: usize) -> i8 {
        debug_assert!(self.columns[index].field_type == ExFieldType::Int8);
        self.data_slice(index).read_int_be()
    }

    pub fn uint8(&self, index: usize) -> u8 {
        debug_assert!(self.columns[index].field_type == ExFieldType::UInt8);
        self.data_slice(index).read_int_be()
    }

    pub fn int16(&self, index: usize) -> i16 {
        debug_assert!(self.columns[index].field_type == ExFieldType::Int16);
        self.data_slice(index).read_int_be()
    }

    pub fn uint16(&self, index: usize) -> u16 {
        debug_assert!(self.columns[index].field_type == ExFieldType::UInt16);
        self.data_slice(index).read_int_be()
    }

    pub fn int32(&self, index: usize) -> i32 {
        debug_assert!(self.columns[index].field_type == ExFieldType::Int32);
        self.data_slice(index).read_int_be()
    }

    pub fn uint32(&self, index: usize) -> u32 {
        debug_assert!(self.columns[index].field_type == ExFieldType::UInt32);
        self.data_slice(index).read_int_be()
    }

    pub fn float(&self, index: usize) -> f32 {
        debug_assert!(self.columns[index].field_type == ExFieldType::Float);
        self.data_slice(index).read_float_be()
    }

    pub fn quad(&self, index: usize) -> (u16, u16, u16, u16) {
        debug_assert!(self.columns[index].field_type == ExFieldType::Quad);
        let data = self.data_slice(index);

        (
            (&data[0..]).read_int_be(),
            (&data[2..]).read_int_be(),
            (&data[4..]).read_int_be(),
            (&data[6..]).read_int_be(),
        )
    }

    fn data_slice(&self, index: usize) -> &[u8] {
        let data_offset = self.columns[index].offset as usize;

        &self.data[data_offset..]
    }
}
