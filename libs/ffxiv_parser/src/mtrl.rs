use core::mem::size_of;

use sqpack_reader::{Package, Result};
use util::{cast, cast_array, SliceByteOrderExt, StrExt};

#[repr(C)]
struct MtrlHeader {
    version: u32,
    file_sizes: u16,
    color_table_size: u16,
    strings_size: u16,
    shader_name_offset: u16,
    texture_count: u8,
    map_count: u8,
    color_set_count: u8,
    unk_size: u8,
}

#[repr(u32)]
#[derive(Eq, PartialEq)]
pub enum MtrlParameterId {
    Normal = 0x0C5EC1F1,
    Mask = 0x8A4E82B6,
    Diffuse = 0x115306BE,
    Specular = 0x2B99E025,
    Catchlight = 0xFEA0F3D2,
}

#[repr(C)]
pub struct MtrlParameter {
    pub id: MtrlParameterId,
    _unk1: u16,
    _unk2: u16,
    pub texture_index: u32,
}

#[repr(C)]
struct MtrlMetadataHeader {
    data_size: u16,
    unk_struct1_count: u16,
    unk_struct2_count: u16,
    parameter_count: u16,
    _unk1: u16,
    _unk2: u16,
}

pub struct Mtrl {
    data: Vec<u8>,
    strings_offset: usize,
    color_table_offset: usize,
    metadata_header_offset: usize,
    parameters_offset: usize,
}

impl Mtrl {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        let header = cast::<MtrlHeader>(&data);
        assert_eq!(header.version, 0x0103_0000);

        let base_offset = size_of::<MtrlHeader>();
        let strings_offset = base_offset + ((header.texture_count + header.map_count + header.color_set_count) as usize) * size_of::<u32>();
        let color_table_offset = strings_offset + header.strings_size as usize;
        let metadata_header_offset = color_table_offset + header.color_table_size as usize + header.unk_size as usize;
        let metadata_header = cast::<MtrlMetadataHeader>(&data[metadata_header_offset..]);
        let parameters_offset = metadata_header_offset
            + size_of::<MtrlMetadataHeader>()
            + 8 * metadata_header.unk_struct1_count as usize
            + 8 * metadata_header.unk_struct2_count as usize;

        Ok(Self {
            data,
            strings_offset,
            color_table_offset,
            metadata_header_offset,
            parameters_offset,
        })
    }

    pub fn texture_files(&self) -> Vec<String> {
        let header = cast::<MtrlHeader>(&self.data);

        let base_offset = size_of::<MtrlHeader>();

        (0..header.texture_count)
            .map(|x| {
                let value = (&self.data[base_offset + (x as usize) * size_of::<u32>()..]).to_int_le::<u32>();
                let offset = value & 0xffff;
                let flag = value >> 16;

                let path = str::from_null_terminated_utf8(&self.data[self.strings_offset + offset as usize..]).unwrap();
                if path == "dummy.tex" {
                    "common/graphics/texture/dummy.tex".to_owned()
                } else if flag & 0x8000 != 0 {
                    let separator = path.rfind('/').unwrap() + 1;
                    format!("{}--{}", &path[..separator], &path[separator..])
                } else {
                    path.to_owned()
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn parameters(&self) -> &[MtrlParameter] {
        let metadata_header = cast::<MtrlMetadataHeader>(&self.data[self.metadata_header_offset..]);
        let parameters = cast_array::<MtrlParameter>(&self.data[self.parameters_offset..]);

        &parameters[..metadata_header.parameter_count as usize]
    }

    pub fn color_table(&self) -> &[u8] {
        let header = cast::<MtrlHeader>(&self.data);
        let offset = self.color_table_offset + size_of::<u32>();

        &self.data[offset..offset + header.color_table_size as usize]
    }

    pub fn shader_name(&self) -> &str {
        let header = cast::<MtrlHeader>(&self.data);

        str::from_null_terminated_utf8(&self.data[self.strings_offset + header.shader_name_offset as usize..]).unwrap()
    }
}
