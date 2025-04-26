use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::mem::size_of;

use glam::Mat4;
use hashbrown::HashMap;

use sqpack::{Package, Result};
use util::{StrExt, cast, cast_array};

#[repr(C)]
struct PreBoneDeformerItem {
    body_id: u16,
    link_index: u16,
    data_offset: u32,
    _unk: u32,
}

#[repr(C)]
struct PreBoneDeformerHeader {
    count: u32,
}

#[repr(C)]
struct PreBoneDeformerLink {
    next_index: i16,
    _unk1: u16,
    _unk2: u16,
    next_item_index: u16,
}

// PreBoneDeformer
pub struct Pbd {
    data: Vec<u8>,
}

impl Pbd {
    pub async fn new(package: &dyn Package) -> Result<Self> {
        let data = package.read_file("chara/xls/bonedeformer/human.pbd").await?;

        Ok(Self { data })
    }

    pub fn get_deform_matrices(&self, from_id: u16, to_id: u16) -> HashMap<String, Mat4> {
        if from_id == to_id {
            return HashMap::new();
        }

        let header = cast::<PreBoneDeformerHeader>(&self.data);
        let items = &cast_array::<PreBoneDeformerItem>(&self.data[size_of::<PreBoneDeformerHeader>()..])[..header.count as usize];

        let item = items.iter().find(|x| x.body_id == from_id);
        if item.is_none() {
            return HashMap::new();
        }
        let mut item = item.unwrap();

        let base_offset = size_of::<PreBoneDeformerHeader>();
        let link_base_offset = base_offset + size_of::<PreBoneDeformerItem>() * header.count as usize;
        let links = cast_array::<PreBoneDeformerLink>(&self.data[link_base_offset..]);

        let mut next = &links[item.link_index as usize];

        if next.next_index == -1 {
            return HashMap::new();
        }

        let mut result = HashMap::new();
        loop {
            let string_offsets_base = item.data_offset as usize + size_of::<u32>();

            let bone_name_count = *cast::<u32>(&self.data[item.data_offset as usize..]) as usize;
            let matrices_base = string_offsets_base + (bone_name_count + bone_name_count % 2) * 2;

            let strings_offset = cast_array::<u16>(&self.data[string_offsets_base..]);
            let matrices = cast_array::<[f32; 12]>(&self.data[matrices_base..]);

            for i in 0..bone_name_count {
                let string_offset = item.data_offset as usize + strings_offset[i] as usize;
                let bone_name = str::from_null_terminated_utf8(&self.data[string_offset..]).unwrap();
                let matrix = matrices[i];

                let entry = result.entry(bone_name.to_owned()).or_insert(Mat4::IDENTITY);

                *entry *= Mat4::from_cols_array(&[
                    matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5], matrix[6], matrix[7], matrix[8], matrix[9], matrix[10],
                    matrix[11], 0., 0., 0., 1.,
                ]);
            }

            next = &links[next.next_index as usize];
            item = &items[next.next_item_index as usize];

            if item.body_id == to_id {
                break;
            }
        }

        result
    }
}
