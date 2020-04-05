use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use bytes::{Buf, Bytes};
use nom::number::complete::{le_f32, le_u32};
use nom::{do_parse, named, tag};
use serde::Serialize;

use sqpack_reader::{Package, Result};
use util::{parse, StrExt};

struct LgbHeader {
    pub file_size: u32,
}

impl LgbHeader {
    pub const SIZE: usize = 20;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"LGB1")   >>
            file_size:      le_u32          >>
            /* unk1: */     le_u32          >>
            /* magic: */    tag!(b"LGP1")   >>
            /* unk2: */     le_u32          >>
            (Self {
                file_size,
            })
        )
    );
}

struct LgbResourceHeader {
    pub name_offset: u32,
    pub entry_count: u32,
}

impl LgbResourceHeader {
    pub const SIZE: usize = 16;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* unk1: */     le_u32  >>
            name_offset:    le_u32  >>
            /* unk2: */     le_u32  >>
            entry_count:    le_u32  >>
            (Self {
                name_offset,
                entry_count,
            })
        )
    );
}

struct LgbResourceEntry {
    name_offset: u32,
    items_offset: u32,
    item_count: u32,
}

impl LgbResourceEntry {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* unk1: */     le_u32  >>
            name_offset:    le_u32  >>
            items_offset:   le_u32  >>
            item_count:     le_u32  >>
            /* unk2: */     le_u32  >>
            /* unk3: */     le_u32  >>
            /* unk4: */     le_u32  >>
            /* unk5: */     le_u32  >>
            /* unk6: */     le_u32  >>
            /* unk7: */     le_u32  >>
            /* unk8: */     le_u32  >>
            /* unk9: */     le_u32  >>
            /* unk10: */    le_u32  >>
            (Self {
                name_offset,
                items_offset,
                item_count,
            })
        )
    );
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LayerGroupResourceItem {
    EventNpc {
        #[serde(rename = "type")]
        item_type: u32,
        id: u32,
        x: f32,
        y: f32,
        z: f32,
        npc_id: u32,
    },
    Unk {
        #[serde(rename = "type")]
        item_type: u32,
    },
}

impl LayerGroupResourceItem {
    pub fn parse(data: &[u8]) -> Self {
        let item_type = (&data[0..]).get_u32_le();
        match item_type {
            8 => Self::parse_eventnpc(data).unwrap().1,
            _ => Self::parse_unk(data).unwrap().1,
        }
    }

    #[rustfmt::skip]
    named!(parse_eventnpc<Self>,
        do_parse!(
            item_type:      le_u32  >>
            id:             le_u32  >>
            /* unk1: */     le_u32  >>
            x:              le_f32  >>
            y:              le_f32  >>
            z:              le_f32  >>
            /* unk2: */     le_f32  >>
            /* unk3: */     le_f32  >>
            /* unk4: */     le_f32  >>
            /* unk5: */     le_f32  >>
            /* unk6: */     le_f32  >>
            /* unk7: */     le_f32  >>
            npc_id:         le_u32  >>
            (LayerGroupResourceItem::EventNpc {
                item_type,
                id,
                x,
                y,
                z,
                npc_id,
            })
        )
    );

    #[rustfmt::skip]
    named!(parse_unk<Self>,
        do_parse!(
            item_type:   le_u32  >>
            (LayerGroupResourceItem::Unk {
                item_type,
            })
        )
    );
}

// LayerGroupResource
pub struct Lgb {
    pub name: String,
    pub entries: BTreeMap<String, Vec<LayerGroupResourceItem>>,
}

impl Lgb {
    pub async fn new(package: &dyn Package, path: &str) -> Result<Self> {
        let data: Bytes = package.read_file(path).await?;

        let _ = parse!(data, LgbHeader);
        let resource_header = parse!(data[LgbHeader::SIZE..], LgbResourceHeader);
        let name = str::from_null_terminated_utf8(&data[LgbHeader::SIZE + resource_header.name_offset as usize..])
            .unwrap()
            .to_owned();

        let base_offset = LgbHeader::SIZE + LgbResourceHeader::SIZE;
        let entries = (0..resource_header.entry_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * core::mem::size_of::<u32>();
                let data_offset = (&data[offset..]).get_u32_le();

                Self::parse_entry(&data, base_offset + data_offset as usize)
            })
            .collect::<BTreeMap<_, _>>();

        Ok(Self { name, entries })
    }

    fn parse_entry(data: &Bytes, offset: usize) -> (String, Vec<LayerGroupResourceItem>) {
        let entry = parse!(&data[offset..], LgbResourceEntry);
        let name = str::from_null_terminated_utf8(&data[offset + entry.name_offset as usize..])
            .unwrap()
            .to_owned();

        let base_offset = offset + entry.items_offset as usize;
        let items = (0..entry.item_count)
            .map(|i| {
                let offset = base_offset + (i as usize) * core::mem::size_of::<u32>();
                let data_offset = (&data[offset..]).get_u32_le();

                LayerGroupResourceItem::parse(&data[base_offset + data_offset as usize..])
            })
            .collect::<Vec<_>>();

        (name, items)
    }
}
