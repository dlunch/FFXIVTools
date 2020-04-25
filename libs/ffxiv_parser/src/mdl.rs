use sqpack_reader::{Package, Result};
use util::cast_array;

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BufferItemType {
    Float1 = 0,
    Float2 = 1,
    Float3 = 2,
    Float4 = 3,
    UByte4 = 5,
    Short2 = 6,
    Short4 = 7,
    UByte4n = 8,
    Short2n = 9,
    Short4n = 10,
    Half2 = 13,
    Half4 = 14,
}

impl BufferItemType {
    pub fn component_count(self) -> usize {
        match self {
            BufferItemType::Float1 => 1,
            BufferItemType::Float2 => 2,
            BufferItemType::Float3 => 3,
            BufferItemType::Float4 => 4,
            BufferItemType::UByte4 => 4,
            BufferItemType::Short2 => 2,
            BufferItemType::Short4 => 4,
            BufferItemType::UByte4n => 4,
            BufferItemType::Short2n => 2,
            BufferItemType::Short4n => 4,
            BufferItemType::Half2 => 2,
            BufferItemType::Half4 => 4,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BufferItemUsage {
    Position = 0,
    BoneWeight = 1,
    BoneIndex = 2,
    Normal = 3,
    TexCoord = 4,
    Tangent = 5,
    Bitangent = 6,
    Color = 7,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferItem {
    pub buffer: u8,
    pub offset: u8,
    pub item_type: BufferItemType,
    pub usage: BufferItemUsage,
    _unk: u32,
}

pub struct Mdl {
    data: Vec<u8>,
}

impl Mdl {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        Ok(Self { data })
    }

    pub fn buffer_items(&self) -> impl Iterator<Item = &BufferItem> {
        let base_offset = 0x44;

        let elements = cast_array::<BufferItem>(&self.data[base_offset..]);
        elements.iter().take_while(|x| x.buffer != 255)
    }
}
