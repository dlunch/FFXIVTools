use alloc::{borrow::ToOwned, string::String, vec::Vec};

use bytes::{Buf, Bytes};
use nom::number::complete::le_u32;
use nom::{do_parse, named, tag};

use sqpack_reader::{Package, Result};
use util::{parse, StrExt};

pub struct LvbHeader {
    pub file_size: u32,
    pub header_size: u32,
}

impl LvbHeader {
    pub const SIZE: usize = 20;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"LVB1")   >>
            file_size:      le_u32          >>
            /* unk1: */     le_u32          >>
            /* magic: */    tag!(b"SCN1")   >>
            header_size:    le_u32          >>
            (Self {
                file_size,
                header_size
            })
        )
    );
}

pub struct LvbEntries {
    pub entry1_offset: u32,
    pub entry2_offset: u32,
    pub entry3_offset: u32,
    pub entry4_offset: u32,
    pub entry4_count: u32,
    pub entry5_offset: u32,
}

impl LvbEntries {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            entry1_offset:  le_u32  >>
            /* unk1: */     le_u32  >>
            /* unk2: */     le_u32  >>
            entry2_offset:  le_u32  >>
            entry3_offset:  le_u32  >>
            entry4_offset:  le_u32  >>
            entry4_count:   le_u32  >>
            /* unk3: */     le_u32  >>
            entry5_offset:  le_u32  >>
            (Self {
                entry1_offset,
                entry2_offset,
                entry3_offset,
                entry4_offset,
                entry4_count,
                entry5_offset,
            })
        )
    );
}

// LevelSceneResource
pub struct Lvb {
    pub lgb_paths: Vec<String>,
}

impl Lvb {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data: Bytes = package.read_file(&format!("bg/{}.lvb", path)).await?;

        let _ = parse!(data, LvbHeader);
        let entries = parse!(data[LvbHeader::SIZE..], LvbEntries);

        let entry4_base = LvbHeader::SIZE + entries.entry4_offset as usize;
        let lgb_paths = (0..entries.entry4_count as usize)
            .map(|x| {
                let offset = entry4_base + x * core::mem::size_of::<u32>();
                let string_offset = entry4_base + (&data[offset..]).get_u32_le() as usize;

                str::from_null_terminated_utf8(&data[string_offset..]).unwrap().to_owned()
            })
            .collect::<Vec<_>>();

        Ok(Self { lgb_paths })
    }
}
