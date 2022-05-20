use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::mem::size_of;

use sqpack::{Package, Result};
use util::{cast, SliceByteOrderExt, StrExt};

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
    pub lgb_entry_offset: u32,
    pub lgb_entry_count: u32,
    _unk3: u32,
    pub entry5_offset: u32,
}

// LevelSceneResource
pub struct Lvb {
    pub lgb_paths: Vec<String>,
}

impl Lvb {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data = package.read_file(path).await?;

        let _ = cast::<LvbHeader>(&data);
        let entries = cast::<LvbEntries>(&data[size_of::<LvbHeader>()..]);

        let lgb_entry_base = size_of::<LvbHeader>() + entries.lgb_entry_offset as usize;
        let lgb_paths = (0..entries.lgb_entry_count as usize)
            .map(|x| {
                let offset = lgb_entry_base + x * size_of::<u32>();
                let string_offset = (&data[offset..]).to_int_le::<u32>() as usize;

                str::from_null_terminated_utf8(&data[lgb_entry_base + string_offset..])
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>();

        Ok(Self { lgb_paths })
    }
}
