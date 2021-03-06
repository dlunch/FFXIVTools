use alloc::vec::Vec;
use core::mem::size_of;

use sqpack::{Package, Result};
use util::{cast, cast_array};

#[repr(C)]
struct EquipmentDeformerParameterHeader {
    _unk: u16,
    row_count: u16,
    offset: u16,
}

// EquipmentDeformerParameter
pub struct Eqdp {
    data: Vec<u8>,
}

impl Eqdp {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        Ok(Self { data })
    }

    pub fn has_model(&self, model_id: u16, model_part: u8) -> bool {
        let header = cast::<EquipmentDeformerParameterHeader>(&self.data);
        let data = cast_array::<u16>(&self.data[size_of::<EquipmentDeformerParameterHeader>()..]);

        let row_index = model_id % header.row_count;
        let offset = data[(model_id / header.row_count) as usize];
        if offset == 65535 {
            return false;
        }
        let deformer_data = data[(row_index + header.offset + offset) as usize];

        match model_part {
            0 => (deformer_data & 0x2) != 0,
            1 => (deformer_data & 0x8) != 0,
            2 => (deformer_data & 0x20) != 0,
            3 => (deformer_data & 0x80) != 0,
            4 => (deformer_data & 0x200) != 0,
            _ => false,
        }
    }
}
