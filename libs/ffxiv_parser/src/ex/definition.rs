use byteorder::{BigEndian, ByteOrder, LittleEndian};
use zerocopy::{
    byteorder::{U16, U32},
    FromBytes,
};

use crate::Language;

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum ExRowType {
    Single = 1,
    Multi = 2,
}

impl ExRowType {
    pub fn from(raw: u16) -> Self {
        match raw {
            1 => ExRowType::Single,
            2 => ExRowType::Multi,

            _ => panic!(),
        }
    }
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExhHeader {
    _magic: [u8; 4],
    pub version: U16<BigEndian>,
    pub row_size: U16<BigEndian>,
    pub column_count: U16<BigEndian>,
    pub page_count: U16<BigEndian>,
    pub language_count: U16<BigEndian>,
    _unk1: u16,
    pub row_type: U16<BigEndian>,
    _unk2: u16,
    pub item_count: U32<BigEndian>,
    _unk3: u32,
    _unk4: u32,
}

#[derive(FromBytes, Clone)]
#[repr(C)]
pub struct ExhColumnDefinition {
    pub field_type: U16<BigEndian>,
    pub offset: U16<BigEndian>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExhPage {
    pub start: u32,
    pub count: u32,
}

impl ExhPage {
    pub fn from(raw: &[u8]) -> Self {
        let start = BigEndian::read_u32(raw);
        let count = BigEndian::read_u32(&raw[core::mem::size_of::<u32>()..]);

        Self { start, count }
    }
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExdHeader {
    _magic: [u8; 4],
    pub version: U16<BigEndian>,
    _unk1: u16,
    pub row_size: U32<BigEndian>,
    pub data_size: U32<BigEndian>,
    _unk2: u32,
    _unk3: u32,
    _unk4: u32,
    _unk5: u32,
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExdRow {
    pub index: U32<BigEndian>,
    pub offset: U32<BigEndian>,
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExdMultiRowDataItemHeader {
    pub sub_index: U16<BigEndian>,
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExdMultiRowDataHeader {
    pub length: U32<BigEndian>,
    pub count: U16<BigEndian>,
}

#[derive(FromBytes)]
#[repr(C)]
pub struct ExdDataHeader {
    pub length: U32<BigEndian>,
    _unk: u16,
}

impl Language {
    pub fn from(raw: &[u8]) -> Self {
        match LittleEndian::read_u16(raw) {
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
    pub fn from(raw: u16) -> Self {
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
