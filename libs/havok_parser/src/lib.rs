use std::str;

use bitflags::bitflags;
use log::debug;

use util::SliceByteOrderExt;

bitflags! {
    struct HavokType: u32 {
        const BYTE = 1;
        const INT = 2;
        const REAL = 3;
        const VEC4 = 4;
        const VEC8 = 5;
        const VEC12 = 6;
        const VEC16 = 7;
        const OBJECT = 8;
        const STRUCT = 9;
        const STRING = 10;

        const ARRAY = 0x10;
        const ARRAYBYTE = Self::ARRAY.bits | Self::BYTE.bits;
        const ARRAYINT = Self::ARRAY.bits | Self::INT.bits;
        const ARRAYREAL = Self::ARRAY.bits | Self::REAL.bits;
        const ARRAYVEC4 = Self::ARRAY.bits | Self::VEC4.bits;
        const ARRAYVEC8 = Self::ARRAY.bits | Self::VEC8.bits;
        const ARRAYVEC12 = Self::ARRAY.bits | Self::VEC12.bits;
        const ARRAYVEC16 = Self::ARRAY.bits | Self::VEC16.bits;
        const ARRAYREALOBJEC = Self::ARRAY.bits | Self::OBJECT.bits;
        const ARRAYSTRUCT = Self::ARRAY.bits | Self::STRUCT.bits;
        const ARRAYSTRING = Self::ARRAY.bits | Self::STRING.bits;

        const TUPLE = 0x20;
        const TUPLEBYTE = Self::TUPLE.bits | Self::BYTE.bits;
        const TUPLEINT = Self::TUPLE.bits | Self::INT.bits;
        const TUPLEREAL = Self::TUPLE.bits | Self::REAL.bits;
        const TUPLEVEC4 = Self::TUPLE.bits | Self::VEC4.bits;
        const TUPLEVEC8 = Self::TUPLE.bits | Self::VEC8.bits;
        const TUPLEVEC12 = Self::TUPLE.bits | Self::VEC12.bits;
        const TUPLEVEC16 = Self::TUPLE.bits | Self::VEC16.bits;
        const TUPLEOBJECT = Self::TUPLE.bits | Self::OBJECT.bits;
        const TUPLESTRUCT = Self::TUPLE.bits | Self::STRUCT.bits;
        const TUPLESTRING = Self::TUPLE.bits | Self::STRING.bits;
    }
}

#[repr(i8)]
enum HavokTagType {
    Eof = -1,
    Invalid = 0,
    FileInfo = 1,
    Metadata = 2,
    Object = 3,
    ObjectRemember = 4,
    Backref = 5,
    ObjectNull = 6,
    FileEnd = 7,
}

impl HavokTagType {
    fn from(raw: u8) -> Self {
        match raw {
            255 => HavokTagType::Eof,
            0 => HavokTagType::Invalid,
            1 => HavokTagType::FileInfo,
            2 => HavokTagType::Metadata,
            3 => HavokTagType::Object,
            4 => HavokTagType::ObjectRemember,
            5 => HavokTagType::Backref,
            6 => HavokTagType::ObjectNull,
            7 => HavokTagType::FileEnd,
            _ => panic!(),
        }
    }
}

type HavokInteger = i32;

pub struct HavokObjectMetadata {}

#[allow(clippy::new_without_default)]
impl HavokObjectMetadata {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct HavokObject {}

#[allow(clippy::new_without_default)]
impl HavokObject {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct HavokBinaryTagFileReader<'a> {
    file_version: u8,
    remembered_strings: Vec<String>,
    remembered_metadatas: Vec<HavokObjectMetadata>,
    remembered_objects: Vec<HavokObject>,
    data: &'a [u8],
    cursor: usize,
}

impl<'a> HavokBinaryTagFileReader<'a> {
    pub fn read(data: &'a [u8]) -> HavokObject {
        let mut reader = Self::new(data);

        reader.do_read()
    }

    fn new(data: &'a [u8]) -> Self {
        let file_version = 0;
        let remembered_strings = vec!["string".to_owned(), "".to_owned()];
        let remembered_metadatas = vec![HavokObjectMetadata::new()];
        let remembered_objects = Vec::new();

        Self {
            file_version,
            remembered_strings,
            remembered_metadatas,
            remembered_objects,
            data,
            cursor: 0,
        }
    }

    fn do_read(&mut self) -> HavokObject {
        let signature1 = (&self.data[0..4]).to_int_le::<u32>();
        let signature2 = (&self.data[4..8]).to_int_le::<u32>();
        if signature1 != 0xCAB0_0D1E || signature2 != 0xD011_FACE {
            panic!()
        }
        self.cursor = 8;

        loop {
            let tag_type = HavokTagType::from(self.read_packed_int() as u8);
            match tag_type {
                HavokTagType::FileInfo => {
                    self.file_version = self.read_packed_int() as u8;
                    if self.file_version != 3 {
                        panic!("Unimplemented version");
                    }
                    debug!("version {}", self.file_version);
                    self.remembered_objects.push(HavokObject::new())
                }
                HavokTagType::Metadata => {
                    let metadata = self.read_metadata();
                    self.remembered_metadatas.push(metadata);
                }
                _ => break,
            }
        }

        HavokObject::new()
    }

    fn read_metadata(&mut self) -> HavokObjectMetadata {
        let name = self.read_string();
        debug!("metadata {}", name);

        HavokObjectMetadata::new()
    }

    fn read_string(&mut self) -> &str {
        let length = self.read_packed_int();
        if length < 0 {
            return &self.remembered_strings[-length as usize];
        }

        self.remembered_strings
            .push(str::from_utf8(&self.data[self.cursor..self.cursor + length as usize]).unwrap().to_owned());
        self.cursor += length as usize;

        &self.remembered_strings[self.remembered_strings.len() - 1]
    }

    fn read_byte(&mut self) -> u8 {
        let result = self.data[self.cursor];
        self.cursor += 1;

        result
    }

    fn read_packed_int(&mut self) -> HavokInteger {
        let mut byte = self.read_byte();

        let mut result = ((byte & 0x7f) >> 1) as u32;
        let neg = byte & 1;

        let mut shift = 6;
        while byte & 0x80 != 0 {
            byte = self.read_byte();

            result |= ((byte as u32) & 0xffff_ff7f) << shift;
            shift += 7;
        }
        if neg == 1 {
            -(result as HavokInteger)
        } else {
            result as HavokInteger
        }
    }
}
