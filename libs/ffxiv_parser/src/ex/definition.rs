use nom::number::complete::{be_u16, be_u32, le_u16};
use nom::{do_parse, named, tag, take, IResult};

use crate::Language;

#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(u16)]
pub enum ExRowType {
    Single = 1,
    Multi = 2,
}

pub struct ExhHeader {
    pub version: u16,
    pub row_size: u16,
    pub column_count: u16,
    pub page_count: u16,
    pub language_count: u16,
    pub row_type: ExRowType,
    pub item_count: u32,
}

impl ExhHeader {
    pub const SIZE: usize = 32;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"EXHF")   >>
            version:        be_u16          >>
            row_size:       be_u16          >>
            column_count:   be_u16          >>
            page_count:     be_u16          >>
            language_count: be_u16          >>
            /* unk1: */     be_u16          >>
            row_type:       be_u16          >>
            /* unk2: */     be_u16          >>
            item_count:     be_u32          >>
            /* unk3: */     be_u32          >>
            /* unk4: */     be_u32          >>
            (Self {
                version,
                row_size,
                column_count,
                page_count,
                language_count,
                row_type: match row_type {
                    1 => ExRowType::Single,
                    2 => ExRowType::Multi,
                    _ => panic!()
                },
                item_count,
            })
        )
    );
}

pub struct ExhColumnDefinition {
    pub field_type: ExFieldType,
    pub offset: u16,
}

impl ExhColumnDefinition {
    pub const SIZE: usize = 4;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            field_type: be_u16  >>
            offset:     be_u16  >>
            (Self {
                field_type: ExFieldType::parse(field_type),
                offset,
            })
        )
    );
}

#[derive(Copy, Clone)]
pub struct ExhPage {
    pub start: u32,
    pub count: u32,
}

impl ExhPage {
    pub const SIZE: usize = 8;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            start:  be_u32  >>
            count:  be_u32  >>
            (Self {
                start,
                count,
            })
        )
    );
}

pub struct ExdHeader {
    pub version: u16,
    pub row_size: u32,
    pub data_size: u32,
}

impl ExdHeader {
    pub const SIZE: usize = 32;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"EXDF")   >>
            version:        be_u16          >>
            /* unk1: */     be_u16          >>
            row_size:       be_u32          >>
            data_size:      be_u32          >>
            /* unk2: */     be_u32          >>
            /* unk3: */     be_u32          >>
            /* unk4: */     be_u32          >>
            /* unk5: */     be_u32          >>
            (Self {
                version,
                row_size,
                data_size,
            })
        )
    );
}

pub struct ExdRow {
    pub index: u32,
    pub offset: u32,
}

impl ExdRow {
    pub const SIZE: usize = 8;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            index:  be_u32  >>
            offset: be_u32  >>
            (Self {
                index,
                offset,
            })
        )
    );
}

pub struct ExdMultiRowDataItem<'a> {
    pub sub_index: u16,
    pub data: &'a [u8],
}

impl<'a> ExdMultiRowDataItem<'a> {
    #[rustfmt::skip]
    pub fn parse(input: &'a [u8], row_size: usize) -> IResult<&'a [u8], Self> {
        do_parse!(
            input,
            sub_index:  be_u16          >>
            data:       take!(row_size) >>
            (Self {
                sub_index,
                data,
            })
        )
    }
}

pub struct ExdMultiRowData<'a> {
    pub length: u32,
    pub count: u16,
    pub data: &'a [u8],
}

impl<'a> ExdMultiRowData<'a> {
    #[rustfmt::skip]
    pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        do_parse!(
            input,
            length: be_u32          >>
            count:  be_u16          >>
            data:   take!(length)   >>
            (Self {
                length,
                count,
                data,
            })
        )
    }
}

pub struct ExdData<'a> {
    pub length: u32,
    pub data: &'a [u8],
}

impl<'a> ExdData<'a> {
    #[rustfmt::skip]
    pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        do_parse!(
            input,
            length:     be_u32          >>
            /* unk: */  be_u16          >>
            data:       take!(length)   >>
            (Self {
                length,
                data,
            })
        )
    }
}

impl Language {
    pub const SIZE: usize = core::mem::size_of::<u16>();

    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let raw = le_u16(input)?;

        let result = match raw.1 {
            0 => Language::None,
            1 => Language::Japanese,
            2 => Language::English,
            3 => Language::Deutsch,
            4 => Language::French,
            5 => Language::ChineseSimplified,
            6 => Language::ChineseTraditional,
            7 => Language::Korean,
            _ => panic!(),
        };

        Ok((raw.0, result))
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
    pub fn parse(raw: u16) -> Self {
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
