use alloc::{borrow::ToOwned, collections::BTreeMap, string::String, vec::Vec};
use core::mem::size_of;

use bytes::{Buf, Bytes};
use serde::Serialize;
use zerocopy::{FromBytes, LayoutVerified};

use sqpack_reader::{Package, Result};
use util::{cast, StrExt};

#[derive(FromBytes)]
#[repr(C)]
struct LgbHeader {
    _magic1: [u8; 4],
    pub file_size: u32,
    _unk1: u32,
    _magic2: [u8; 4],
    _unk2: u32,
}

#[derive(FromBytes)]
#[repr(C)]
struct LgbResourceHeader {
    _unk1: u32,
    pub name_offset: u32,
    _unk2: u32,
    pub entry_count: u32,
}

#[derive(FromBytes)]
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

#[derive(FromBytes, Serialize, Clone)]
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

#[derive(FromBytes, Serialize, Clone)]
#[repr(C)]
pub struct LayerGroupResourceItemUnk {
    #[serde(rename = "type")]
    pub item_type: u32,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LayerGroupResourceItem {
    EventNpc(LayerGroupResourceItemEventNpc),
    Unk(LayerGroupResourceItemUnk),
}

impl LayerGroupResourceItem {
    pub fn from(raw: &[u8]) -> Self {
        let item_type = (&raw[..]).get_u32_le();

        match item_type {
            8 => LayerGroupResourceItem::EventNpc(cast!(raw, LayerGroupResourceItemEventNpc).clone()),
            _ => LayerGroupResourceItem::Unk(cast!(raw, LayerGroupResourceItemUnk).clone()),
        }
    }
}

// LayerGroupResource
pub struct Lgb {
    pub name: String,
    pub entries: BTreeMap<String, Vec<LayerGroupResourceItem>>,
}

impl Lgb {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data: Bytes = package.read_file(path.as_ref()).await?;

        let _ = cast!(data, LgbHeader);
        let resource_header = cast!(&data[size_of::<LgbHeader>()..], LgbResourceHeader);
        let name = str::from_null_terminated_utf8(&data[size_of::<LgbHeader>() + resource_header.name_offset as usize..])
            .unwrap()
            .to_owned();

        let base_offset = size_of::<LgbHeader>() + size_of::<LgbResourceHeader>();
        let entries = (0..resource_header.entry_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * size_of::<u32>();
                let data_offset = (&data[offset..]).get_u32_le();

                Self::parse_entry(&data, base_offset + data_offset as usize)
            })
            .collect::<BTreeMap<_, _>>();

        Ok(Self { name, entries })
    }

    fn parse_entry(data: &Bytes, offset: usize) -> (String, Vec<LayerGroupResourceItem>) {
        let entry = cast!(&data[offset..], LgbResourceEntry);
        let name = str::from_null_terminated_utf8(&data[offset + entry.name_offset as usize..])
            .unwrap()
            .to_owned();

        let base_offset = offset + entry.items_offset as usize;
        let items = (0..entry.item_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * size_of::<u32>();
                let data_offset = (&data[offset..]).get_u32_le();

                LayerGroupResourceItem::from(&data[base_offset + data_offset as usize..])
            })
            .collect::<Vec<_>>();

        (name, items)
    }
}
