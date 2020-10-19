use alloc::vec::Vec;
use core::mem::size_of;

use hashbrown::HashMap;
use sqpack::{Package, Result};
use util::{cast, cast_array};

#[repr(C)]
struct StainingTemplateHeader {
    _magic: u16,
    _version: u16,
    item_count: u16,
    _unk: u16,
}

// StainingTemplate
pub struct Stm {
    data: Vec<u8>,
    template_base: usize,
    template_offsets: HashMap<u16, usize>,
}

impl Stm {
    pub async fn new(package: &dyn Package) -> Result<Self> {
        let data = package.read_file("chara/base_material/stainingtemplate.stm").await?;

        let header = cast::<StainingTemplateHeader>(&data);
        let ids = &cast_array::<u16>(&data[size_of::<StainingTemplateHeader>()..])[..header.item_count as usize];
        let offsets = &cast_array::<u16>(&data[size_of::<StainingTemplateHeader>() + header.item_count as usize * size_of::<u16>()..])
            [..header.item_count as usize];

        let template_base = size_of::<StainingTemplateHeader>() + header.item_count as usize * size_of::<u16>() * 2;
        let template_offsets = ids.iter().cloned().zip(offsets.iter().map(|&x| x as usize)).collect::<HashMap<_, _>>();

        Ok(Self {
            data,
            template_base,
            template_offsets,
        })
    }

    pub fn get(&self, stain_id: u16) -> &[u8] {
        let offsets = self.template_offsets.get(&stain_id).unwrap();

        &self.data[self.template_base + offsets * 2..]
    }
}
