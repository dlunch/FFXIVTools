use alloc::vec::Vec;

use sqpack_reader::{Package, Result};
use util::cast;

#[repr(C)]
struct SkeletonHeader {
    signature: u32,
    version: u32,
}

#[repr(C)]
struct SkeletonHeader12 {
    _signature: u32,
    _version: u32,
    _unk_offset: u16,
    hkx_offset: u16,
    _body_id: u32,
    _mapper_body_id_1: u32,
    _mapper_body_id_2: u32,
    _mapper_body_id_3: u32,
}

#[repr(C)]
struct SkeletonHeader13 {
    _signature: u32,
    _version: u32,
    _unk_offset: u32,
    hkx_offset: u32,
    _unk: u32,
    _body_id: u32,
    _mapper_body_id_1: u32,
    _mapper_body_id_2: u32,
    _mapper_body_id_3: u32,
}

// Skeleton
pub struct Sklb {
    data: Vec<u8>,
    hkx_offset: u32,
}

impl Sklb {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data = package.read_file(path).await?;

        let header = cast::<SkeletonHeader>(&data);
        let hkx_offset;
        if header.version == 0x3132_3030 {
            // '1200'
            let header = cast::<SkeletonHeader12>(&data);
            hkx_offset = header.hkx_offset as u32;
        } else if header.version == 0x3133_3030 || header.version == 0x3133_3031 {
            // '1300' or '1301'
            let header = cast::<SkeletonHeader13>(&data);
            hkx_offset = header.hkx_offset;
        } else {
            panic!()
        }

        Ok(Self { data, hkx_offset })
    }

    pub fn hkx_data(&self) -> &[u8] {
        &self.data[self.hkx_offset as usize..]
    }
}
