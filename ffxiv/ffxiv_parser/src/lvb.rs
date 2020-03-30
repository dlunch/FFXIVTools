mod definition;

use std::io;

use bytes::Buf;

use sqpack_reader::Package;
use util::{parse, StrExt};

use definition::{LvbEntries, LvbHeader};

// LevelSceneResource
pub struct Lvb {
    pub lgb_paths: Vec<String>,
}

impl Lvb {
    pub async fn new(package: &dyn Package, path: &str) -> io::Result<Self> {
        let data = package.read_file(&format!("bg/{}.lvb", path)).await?;

        let _ = parse!(data, LvbHeader);
        let entries = parse!(data[LvbHeader::SIZE..], LvbEntries);

        let mut lgb_paths = Vec::new();

        let entry4_base = LvbHeader::SIZE + entries.entry4_offset as usize;
        let mut cursor = entry4_base;
        for _ in 0..entries.entry4_count {
            let string_offset = entry4_base + (&data[cursor..]).get_u32_le() as usize;
            lgb_paths.push(str::from_null_terminated_utf8(&data[string_offset..]).unwrap().to_owned());

            cursor += std::mem::size_of::<u32>();
        }

        Ok(Self { lgb_paths })
    }
}
