use alloc::vec::Vec;
use core::mem::size_of;

use sqpack::{Package, Result};

use util::{cast, SliceByteOrderExt};

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextureType {
    DXT1 = 0x3420,
    DXT3 = 0x3430,
    DXT5 = 0x3431,
    BGRA = 0x1450,
}

#[repr(C)]
struct TexHeader {
    _unk1: u16,
    header_size: u16,
    texture_type: TextureType,
    _unk2: u8,
    _unk3: u8,
    width: u16,
    height: u16,
    depth: u16,
    mipmap_count: u16,
    _unk4: u32,
    _unk5: u32,
    _unk6: u32,
}

pub struct Tex {
    data: Vec<u8>,
}

impl Tex {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        Ok(Self { data })
    }

    pub fn width(&self) -> u16 {
        let header = cast::<TexHeader>(&self.data);

        header.width
    }

    pub fn height(&self) -> u16 {
        let header = cast::<TexHeader>(&self.data);

        header.height
    }

    pub fn mipmap_count(&self) -> u16 {
        let header = cast::<TexHeader>(&self.data);

        header.mipmap_count
    }

    pub fn texture_type(&self) -> TextureType {
        let header = cast::<TexHeader>(&self.data);

        header.texture_type
    }

    pub fn data(&self, mipmap_index: u16) -> &[u8] {
        let header = cast::<TexHeader>(&self.data);

        let mipmap_begin = self.read_mipmap_offset(mipmap_index);
        let mipmap_end = if mipmap_index == header.mipmap_count - 1 {
            self.data.len()
        } else {
            self.read_mipmap_offset(mipmap_index + 1)
        };

        &self.data[mipmap_begin..mipmap_end]
    }

    fn read_mipmap_offset(&self, mipmap_index: u16) -> usize {
        let mipmap_index = mipmap_index as usize;
        let mipmap_offsets_begin = size_of::<TexHeader>();
        let mipmap_data = &self.data[mipmap_offsets_begin..];
        let mipmap_offset_data = &mipmap_data[mipmap_index * size_of::<u32>()..(mipmap_index + 1) * size_of::<u32>()];

        mipmap_offset_data.to_int_le::<u32>() as usize
    }
}
