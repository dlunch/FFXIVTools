use bitflags::bitflags;

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

// WIP
#[allow(dead_code)]
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

// WIP
#[allow(dead_code)]
type HavokInteger = u32;

pub struct HavokObject {}

#[allow(clippy::new_without_default)]
impl HavokObject {
    pub fn new() -> Self {
        Self {}
    }
}

// WIP
#[allow(dead_code)]
pub struct HavokBinaryTagFileReader<'a> {
    file_version: i8,
    read_strings: Vec<String>,
    read_objects: Vec<HavokObject>,
    data: &'a [u8],
}

impl<'a> HavokBinaryTagFileReader<'a> {
    pub fn read(data: &'a [u8]) -> HavokObject {
        let mut reader = Self::new(data);

        reader.do_read()
    }

    fn new(data: &'a [u8]) -> Self {
        let file_version = -1;
        let read_strings = vec!["string".to_owned(), "".to_owned()];
        let read_objects = vec![HavokObject::new()];

        Self {
            file_version,
            read_strings,
            read_objects,
            data,
        }
    }

    fn do_read(&mut self) -> HavokObject {
        let signature1 = (&self.data[0..4]).to_int_le::<u32>();
        let signature2 = (&self.data[4..8]).to_int_le::<u32>();
        if signature1 != 0xCAB0_0D1E || signature2 != 0xD011_FACE {
            panic!()
        }

        HavokObject::new()
    }
}
