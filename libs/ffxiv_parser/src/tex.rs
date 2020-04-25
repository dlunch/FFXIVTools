use bytes::Bytes;
use sqpack_reader::{Package, Result};

use util::cast;

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
    _unk4: u16,
    _unk5: u16,
    _unk6: u16,
}

pub struct Tex {
    data: Bytes,
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
}
