use alloc::{collections::BTreeMap, vec::Vec};
use core::mem::size_of;

use serde::{Serialize, Serializer};

use sqpack_reader::{Package, Result};
use util::{cast, SliceByteOrderExt, StrExt};

#[repr(C)]
struct LgbHeader {
    _magic1: [u8; 4],
    pub file_size: u32,
    _unk1: u32,
    _magic2: [u8; 4],
    _unk2: u32,
}

#[repr(C)]
struct LgbResourceHeader {
    _unk1: u32,
    pub name_offset: u32,
    _unk2: u32,
    pub entry_count: u32,
}

#[repr(C)]
struct LgbResourceEntry {
    _unk1: u32,
    name_offset: u32,
    items_offset: u32,
    item_count: u32,
    _unk2: u32,
    _unk3: u32,
    _unk4: u32,
    _unk5: u32,
    _unk6: u32,
    _unk7: u32,
    _unk8: u32,
    _unk9: u32,
    _unk10: u32,
}

#[derive(Serialize, Clone)]
#[repr(C)]
pub struct LayerGroupResourceItemEventNpc {
    #[serde(rename = "type")]
    pub item_type: u32,
    pub id: u32,
    #[serde(skip)]
    _unk1: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(skip)]
    _unk2: u32,
    #[serde(skip)]
    _unk3: u32,
    #[serde(skip)]
    _unk4: u32,
    #[serde(skip)]
    _unk5: u32,
    #[serde(skip)]
    _unk6: u32,
    #[serde(skip)]
    _unk7: u32,
    #[serde(rename = "npcId")]
    pub npc_id: u32,
}

#[derive(Serialize, Clone)]
#[repr(C)]
pub struct LayerGroupResourceItemUnk {
    #[serde(rename = "type")]
    pub item_type: u32,
}

pub enum LayerGroupResourceItem<'a> {
    EventNpc(&'a LayerGroupResourceItemEventNpc),
    Unk(&'a LayerGroupResourceItemUnk),
}

impl Serialize for LayerGroupResourceItem<'_> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LayerGroupResourceItem::EventNpc(x) => x.serialize(serializer),
            LayerGroupResourceItem::Unk(x) => x.serialize(serializer),
        }
    }
}

impl<'a> LayerGroupResourceItem<'a> {
    pub fn from(raw: &'a [u8]) -> Self {
        let item_type = raw.to_int_le::<u32>();

        match item_type {
            8 => LayerGroupResourceItem::EventNpc(cast::<LayerGroupResourceItemEventNpc>(raw)),
            _ => LayerGroupResourceItem::Unk(cast::<LayerGroupResourceItemUnk>(raw)),
        }
    }
}

// LayerGroupResource
pub struct Lgb {
    data: Vec<u8>,
    name_offset: u32,
    entry_count: u32,
}

impl Lgb {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        let _ = cast::<LgbHeader>(&data);
        let resource_header = cast::<LgbResourceHeader>(&data[size_of::<LgbHeader>()..]);
        let name_offset = resource_header.name_offset;
        let entry_count = resource_header.entry_count;

        Ok(Self {
            data,
            name_offset,
            entry_count,
        })
    }

    pub fn name(&self) -> &str {
        str::from_null_terminated_utf8(&self.data[size_of::<LgbHeader>() + self.name_offset as usize..]).unwrap()
    }

    pub fn entries<'a>(&'a self) -> BTreeMap<&'a str, Vec<LayerGroupResourceItem<'a>>> {
        let base_offset = size_of::<LgbHeader>() + size_of::<LgbResourceHeader>();
        (0..self.entry_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * size_of::<u32>();
                let data_offset = (&self.data[offset..]).to_int_le::<u32>();

                Self::parse_entry(&self.data[base_offset + data_offset as usize..])
            })
            .collect::<BTreeMap<_, _>>()
    }

    fn parse_entry<'a>(data: &'a [u8]) -> (&'a str, Vec<LayerGroupResourceItem<'a>>) {
        let entry = cast::<LgbResourceEntry>(&data);
        let name = str::from_null_terminated_utf8(&data[entry.name_offset as usize..]).unwrap();

        let base_offset = entry.items_offset as usize;
        let items = (0..entry.item_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * size_of::<u32>();
                let data_offset = (&data[offset..]).to_int_le::<u32>();

                LayerGroupResourceItem::from(&data[base_offset + data_offset as usize..])
            })
            .collect::<Vec<_>>();

        (name, items)
    }
}

impl Serialize for Lgb {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.entries().serialize(serializer)
    }
}
