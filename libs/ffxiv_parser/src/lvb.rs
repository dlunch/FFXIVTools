use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use core::mem::size_of;

use bytes::{Buf, Bytes};

use sqpack_reader::{Package, Result};
use util::{cast, StrExt};

#[repr(C)]
struct LvbHeader {
    _magic1: [u8; 4],
    pub file_size: u32,
    _unk1: u32,
    _magic2: [u8; 4],
    pub header_size: u32,
}

#[repr(C)]
struct LvbEntries {
    pub entry1_offset: u32,
    _unk1: u32,
    _unk2: u32,
    pub entry2_offset: u32,
    pub entry3_offset: u32,
    pub entry4_offset: u32,
    pub entry4_count: u32,
    _unk3: u32,
    pub entry5_offset: u32,
}

// LevelSceneResource
pub struct Lvb {
    pub lgb_paths: Vec<String>,
}

impl Lvb {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data: Bytes = package.read_file(&format!("bg/{}.lvb", path)).await?;

        let _ = cast::<LvbHeader>(&data);
        let entries = cast::<LvbEntries>(&data[size_of::<LvbHeader>()..]);

        let entry4_base = size_of::<LvbHeader>() + entries.entry4_offset as usize;
        let lgb_paths = (0..entries.entry4_count as usize)
            .map(|x| {
                let offset = entry4_base + x * size_of::<u32>();
                let string_offset = entry4_base + (&data[offset..]).get_u32_le() as usize;

                str::from_null_terminated_utf8(&data[string_offset..]).unwrap().to_owned()
            })
            .collect::<Vec<_>>();

        Ok(Self { lgb_paths })
    }
}
