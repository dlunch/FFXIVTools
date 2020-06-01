use core::mem::size_of;

use util::SliceByteOrderExt;

use crate::Language;

#[derive(Clone)]
#[repr(C)]
pub struct U16BE {
    raw: [u8; 2],
}

impl U16BE {
    pub fn get(&self) -> u16 {
        u16::from_be_bytes(self.raw)
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct U32BE {
    raw: [u8; 4],
}

impl U32BE {
    pub fn get(&self) -> u32 {
        u32::from_be_bytes(self.raw)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum ExRowType {
    Single = 1,
    Multi = 2,
}

impl ExRowType {
    pub fn from_raw(raw: u16) -> Self {
        match raw {
            1 => ExRowType::Single,
            2 => ExRowType::Multi,

            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct ExhHeader {
    _magic: [u8; 4],
    pub version: U16BE,
    pub row_size: U16BE,
    pub column_count: U16BE,
    pub page_count: U16BE,
    pub language_count: U16BE,
    _unk1: u16,
    pub row_type: U16BE,
    _unk2: u16,
    pub item_count: U32BE,
    _unk3: u32,
    _unk4: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct ExhColumnDefinition {
    pub field_type: U16BE,
    pub offset: U16BE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExhPage {
    pub start: u32,
    pub count: u32,
}

impl ExhPage {
    pub fn from_raw(raw: &[u8]) -> Self {
        let start = (&raw[..]).to_int_be::<u32>();
        let count = (&raw[size_of::<u32>()..]).to_int_be::<u32>();

        Self { start, count }
    }
}

#[repr(C)]
pub struct ExdHeader {
    _magic: [u8; 4],
    pub version: U16BE,
    _unk1: u16,
    pub row_size: U32BE,
    pub data_size: U32BE,
    _unk2: u32,
    _unk3: u32,
    _unk4: u32,
    _unk5: u32,
}

#[repr(C)]
pub struct ExdRow {
    pub index: U32BE,
    pub offset: U32BE,
}

#[repr(C)]
pub struct ExdMultiRowDataItemHeader {
    pub sub_index: U16BE,
}

#[repr(C)]
pub struct ExdMultiRowDataHeader {
    pub length: U32BE,
    pub count: U16BE,
}

#[repr(C)]
pub struct ExdDataHeader {
    pub length: U32BE,
    _unk: u16,
}

impl Language {
    pub fn from_raw(raw: &[u8]) -> Self {
        match (&raw[..]).to_int_le::<u16>() {
            0 => Language::None,
            1 => Language::Japanese,
            2 => Language::English,
            3 => Language::Deutsch,
            4 => Language::French,
            5 => Language::ChineseSimplified,
            6 => Language::ChineseTraditional,
            7 => Language::Korean,
            _ => panic!(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ExFieldType {
    String = 0,
    Bool = 1,
    Int8 = 2,
    UInt8 = 3,
    Int16 = 4,
    UInt16 = 5,
    Int32 = 6,
    UInt32 = 7,

    Float = 9,
    Quad = 11,
    PackedBool = 25,
}

impl ExFieldType {
    pub fn from_raw(raw: u16) -> Self {
        match raw {
            0 => ExFieldType::String,
            1 => ExFieldType::Bool,
            2 => ExFieldType::Int8,
            3 => ExFieldType::UInt8,
            4 => ExFieldType::Int16,
            5 => ExFieldType::UInt16,
            6 => ExFieldType::Int32,
            7 => ExFieldType::UInt32,
            9 => ExFieldType::Float,
            11 => ExFieldType::Quad,
            25..=core::u16::MAX => ExFieldType::PackedBool,
            _ => panic!(),
        }
    }
}
