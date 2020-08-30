use alloc::vec::Vec;

use sqpack::{Package, Result};
use util::cast;

#[repr(C, packed(1))]
struct PartialAnimationPackHeader {
    signature: u32,
    _unk1: u16,
    _unk2: u16,
    animation_count: u16,
    body_id: u16,
    _unk3: u16,
    header_size: u32,
    hkx_offset: u32,
    footer_offset: u32,
}

// PartialAnimationPack
pub struct Pap {
    data: Vec<u8>,
}

impl Pap {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data = package.read_file(path).await?;

        Ok(Self { data })
    }

    pub fn hkx_data(&self) -> &[u8] {
        let header = cast::<PartialAnimationPackHeader>(&self.data);
        &self.data[header.hkx_offset as usize..]
    }
}
