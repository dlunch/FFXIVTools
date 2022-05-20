use alloc::{vec, vec::Vec};
use core::mem::size_of;

use sqpack::{Package, Result};

use util::{cast, cast_array, SliceByteOrderExt};

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TextureType {
    DXT1 = 0x3420,
    DXT3 = 0x3430,
    DXT5 = 0x3431,
    BGRA = 0x1450,
    RGBA5551 = 0x1441,
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

    pub fn data_rgba(&self, mipmap_index: u16) -> Vec<u8> {
        let data = self.data(mipmap_index);

        match self.texture_type() {
            TextureType::RGBA5551 => Self::convert_5551_to_rgba(data),
            TextureType::DXT1 | TextureType::DXT3 | TextureType::DXT5 => {
                Self::decode_dxtn(self.texture_type(), data, self.width() as usize, self.height() as usize)
            }

            _ => unimplemented!(),
        }
    }

    fn read_mipmap_offset(&self, mipmap_index: u16) -> usize {
        let mipmap_index = mipmap_index as usize;
        let mipmap_offsets_begin = size_of::<TexHeader>();
        let mipmap_data = &self.data[mipmap_offsets_begin..];
        let mipmap_offset_data = &mipmap_data[mipmap_index * size_of::<u32>()..(mipmap_index + 1) * size_of::<u32>()];

        mipmap_offset_data.to_int_le::<u32>() as usize
    }

    fn convert_5551_to_rgba(raw: &[u8]) -> Vec<u8> {
        let raw = cast_array::<u16>(raw);

        raw.iter()
            .flat_map(|i| {
                let b = ((i & 0x1f) * 8) as u8;
                let g = (((i >> 5) & 0x1f) * 8) as u8;
                let r = (((i >> 10) & 0x1f) * 8) as u8;
                let a = (((i >> 15) & 0x1) * 255) as u8;

                [r, g, b, a]
            })
            .collect()
    }

    fn decode_dxtn(format: TextureType, raw: &[u8], width: usize, height: usize) -> Vec<u8> {
        let format = match format {
            TextureType::DXT1 => squish::Format::Bc1,
            TextureType::DXT3 => squish::Format::Bc2,
            TextureType::DXT5 => squish::Format::Bc3,
            _ => unreachable!(),
        };

        let mut result = vec![0; width * height * 4];
        format.decompress(raw, width, height, &mut result);

        result
    }
}
