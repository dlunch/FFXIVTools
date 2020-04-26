use alloc::vec::Vec;

use serde::{ser::SerializeSeq, ser::SerializeTuple, Serialize, Serializer};

use super::definition::{ExFieldType, ExhColumnDefinition};
use crate::ffxiv_string::FFXIVString;

use util::SliceByteOrderExt;

pub enum ExRowItem<'a> {
    String(FFXIVString<'a>),
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

    pub fn all(&self) -> Vec<ExRowItem> {
        (0..self.columns.len()).map(|x| self.index(x)).collect::<Vec<_>>()
    }

    pub fn index(&self, index: usize) -> ExRowItem {
        match ExFieldType::from(self.columns[index].field_type.get()) {
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

    pub fn string(&self, index: usize) -> FFXIVString {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::String);

        let str_offset = self.data_slice(index).to_int_be::<u32>() as usize + self.row_size as usize;

        FFXIVString::new(&self.data[str_offset..])
    }

    pub fn bool(&self, index: usize) -> bool {
        let packed_bool_offset = ExFieldType::PackedBool as u16;
        let field_type_value = ExFieldType::from(self.columns[index].field_type.get()) as u16;

        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Bool || field_type_value >= packed_bool_offset);

        let data;
        if field_type_value >= packed_bool_offset {
            // packed bool
            let packed_data = self.data_slice(index).to_int_be::<u8>();
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
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Int8);
        self.data_slice(index).to_int_be::<i8>()
    }

    pub fn uint8(&self, index: usize) -> u8 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::UInt8);
        self.data_slice(index).to_int_be::<u8>()
    }

    pub fn int16(&self, index: usize) -> i16 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Int16);
        self.data_slice(index).to_int_be::<i16>()
    }

    pub fn uint16(&self, index: usize) -> u16 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::UInt16);
        self.data_slice(index).to_int_be::<u16>()
    }

    pub fn int32(&self, index: usize) -> i32 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Int32);
        self.data_slice(index).to_int_be::<i32>()
    }

    pub fn uint32(&self, index: usize) -> u32 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::UInt32);
        self.data_slice(index).to_int_be::<u32>()
    }

    pub fn float(&self, index: usize) -> f32 {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Float);
        self.data_slice(index).to_float_be::<f32>()
    }

    pub fn quad(&self, index: usize) -> (u16, u16, u16, u16) {
        debug_assert!(ExFieldType::from(self.columns[index].field_type.get()) == ExFieldType::Quad);
        let data = self.data_slice(index);

        (
            (&data[0..]).to_int_be::<u16>(),
            (&data[2..]).to_int_be::<u16>(),
            (&data[4..]).to_int_be::<u16>(),
            (&data[6..]).to_int_be::<u16>(),
        )
    }

    fn data_slice(&self, index: usize) -> &[u8] {
        let data_offset = self.columns[index].offset.get() as usize;

        &self.data[data_offset..]
    }
}

impl Serialize for ExRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rows = self.all();

        let mut seq = serializer.serialize_seq(Some(rows.len()))?;
        for row in rows {
            seq.serialize_element(&row)?;
        }
        seq.end()
    }
}

impl Serialize for ExRowItem<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExRowItem::String(x) => serializer.serialize_str(&x.decode()),
            ExRowItem::Bool(x) => serializer.serialize_bool(*x),
            ExRowItem::Int8(x) => serializer.serialize_i8(*x),
            ExRowItem::UInt8(x) => serializer.serialize_u8(*x),
            ExRowItem::Int16(x) => serializer.serialize_i16(*x),
            ExRowItem::UInt16(x) => serializer.serialize_u16(*x),
            ExRowItem::Int32(x) => serializer.serialize_i32(*x),
            ExRowItem::UInt32(x) => serializer.serialize_u32(*x),
            ExRowItem::Float(x) => serializer.serialize_f32(*x),
            ExRowItem::Quad(x) => {
                let mut tup = serializer.serialize_tuple(4)?;
                tup.serialize_element(&x.0)?;
                tup.serialize_element(&x.1)?;
                tup.serialize_element(&x.2)?;
                tup.serialize_element(&x.3)?;
                tup.end()
            }
        }
    }
}
