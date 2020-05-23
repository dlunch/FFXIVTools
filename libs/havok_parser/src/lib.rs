use std::collections::HashMap;
use std::str;
use std::sync::Arc;

use bitflags::bitflags;
use log::debug;

use util::SliceByteOrderExt;

bitflags! {
    pub struct HavokType: u32 {
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

// WIP
#[allow(dead_code)]
impl HavokType {
    pub fn is_tuple(self) -> bool {
        (self.bits & HavokType::TUPLE.bits) != 0
    }

    pub fn is_array(self) -> bool {
        (self.bits & HavokType::ARRAY.bits) != 0
    }

    pub fn base_type(self) -> HavokType {
        HavokType::from_bits(self.bits & 0x0f).unwrap()
    }

    pub fn is_vec(self) -> bool {
        let base_type = self.base_type();
        base_type == HavokType::VEC4 || base_type == HavokType::VEC8 || base_type == HavokType::VEC12 || base_type == HavokType::VEC16
    }

    pub fn vec_size(self) -> u8 {
        match self.base_type() {
            HavokType::VEC4 => 4,
            HavokType::VEC8 => 8,
            HavokType::VEC12 => 16,
            HavokType::VEC16 => 16,
            _ => panic!(),
        }
    }
}

#[repr(i8)]
enum HavokTagType {
    Eof = -1,
    Invalid = 0,
    FileInfo = 1,
    Type = 2,
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
            2 => HavokTagType::Type,
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

// WIP
#[allow(dead_code)]
pub struct HavokObjectMember {
    name: String,
    member_type: HavokType,
    tuple_size: u32,
    type_name: Option<String>,
}

impl HavokObjectMember {
    pub fn new(name: String, member_type: HavokType, tuple_size: u32, type_name: Option<String>) -> Self {
        Self {
            name,
            member_type,
            tuple_size,
            type_name,
        }
    }
}

// WIP
#[allow(dead_code)]
pub struct HavokObjectType {
    name: String,
    version: u32,
    parent: Option<Arc<HavokObjectType>>,
    members: HashMap<usize, HavokObjectMember>,
}

impl HavokObjectType {
    pub fn new(name: String, version: u32, parent: Option<Arc<HavokObjectType>>, members: HashMap<usize, HavokObjectMember>) -> Self {
        Self {
            name,
            version,
            parent,
            members,
        }
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
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
    remembered_types: Vec<Arc<HavokObjectType>>,
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
        let remembered_types = vec![Arc::new(HavokObjectType::new("base".to_owned(), 0, None, HashMap::new()))];
        let remembered_objects = Vec::new();

        Self {
            file_version,
            remembered_strings,
            remembered_types,
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
                HavokTagType::Type => {
                    let object_type = self.read_type();
                    self.remembered_types.push(Arc::new(object_type));
                }
                _ => break,
            }
        }

        HavokObject::new()
    }

    fn read_type(&mut self) -> HavokObjectType {
        let name = self.read_string();
        let version = self.read_packed_int();
        let parent = self.read_packed_int();
        let member_count = self.read_packed_int();

        let parent = self.remembered_types[parent as usize].clone();
        let members = (0..member_count)
            .map(|x| {
                let member_name = self.read_string();
                let member_type = HavokType::from_bits(self.read_packed_int() as u32).unwrap();

                let tuple_size = if member_type.is_tuple() { self.read_packed_int() } else { 0 };
                let type_name = if member_type.base_type() == HavokType::OBJECT || member_type.base_type() == HavokType::STRUCT {
                    Some(self.read_string())
                } else {
                    None
                };

                let index = parent.member_count() + x as usize;
                let member = HavokObjectMember::new(member_name, member_type, tuple_size as u32, type_name);

                (index, member)
            })
            .collect::<HashMap<_, _>>();

        debug!(
            "type {} members {}",
            name,
            members.values().map(|x| x.name.to_owned()).collect::<Vec<_>>().join(", ")
        );

        HavokObjectType::new(name, version as u32, Some(parent), members)
    }

    fn read_string(&mut self) -> String {
        let length = self.read_packed_int();
        if length < 0 {
            return self.remembered_strings[-length as usize].to_owned();
        }

        let result = str::from_utf8(&self.data[self.cursor..self.cursor + length as usize]).unwrap();
        self.remembered_strings.push(result.to_owned());
        self.cursor += length as usize;

        result.to_owned()
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
