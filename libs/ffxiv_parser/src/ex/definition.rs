use core::mem::size_of;

use util::SliceByteOrderExt;

#[derive(Clone)]
#[repr(C)]
pub struct U16be {
    raw: [u8; 2],
}

impl U16be {
    pub fn get(&self) -> u16 {
        u16::from_be_bytes(self.raw)
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct U32be {
    raw: [u8; 4],
}

impl U32be {
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
    pub version: U16be,
    pub row_size: U16be,
    pub column_count: U16be,
    pub page_count: U16be,
    pub language_count: U16be,
    _unk1: u16,
    pub row_type: U16be,
    _unk2: u16,
    pub item_count: U32be,
    _unk3: u32,
    _unk4: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct ExhColumnDefinition {
    pub field_type: U16be,
    pub offset: U16be,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExhPage {
    pub start: u32,
    pub count: u32,
}

impl ExhPage {
    pub fn from_raw(raw: &[u8]) -> Self {
        let start = raw.to_int_be::<u32>();
        let count = (&raw[size_of::<u32>()..]).to_int_be::<u32>();

        Self { start, count }
    }
}

#[repr(C)]
pub struct ExdHeader {
    _magic: [u8; 4],
    pub version: U16be,
    _unk1: u16,
    pub row_size: U32be,
    pub data_size: U32be,
    _unk2: u32,
    _unk3: u32,
    _unk4: u32,
    _unk5: u32,
}

#[repr(C)]
pub struct ExdRow {
    pub index: U32be,
    pub offset: U32be,
}

#[repr(C)]
pub struct ExdMultiRowDataItemHeader {
    pub sub_index: U16be,
}

#[repr(C)]
pub struct ExdMultiRowDataHeader {
    pub length: U32be,
    pub count: U16be,
}

#[repr(C)]
pub struct ExdDataHeader {
    pub length: U32be,
    _unk: u16,
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
            25..=u16::MAX => ExFieldType::PackedBool,
            _ => panic!(),
        }
    }
}
