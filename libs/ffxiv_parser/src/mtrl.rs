use core::mem::size_of;

use sqpack_reader::{Package, Result};
use util::{cast, SliceByteOrderExt, StrExt};

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
    _unk_size: u8,
}

pub struct Mtrl {
    data: Vec<u8>,
}

impl Mtrl {
    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        let header = cast::<MtrlHeader>(&data);
        assert_eq!(header.version, 0x0103_0000);

        Ok(Self { data })
    }

    pub fn texture_files(&self) -> Vec<String> {
        let header = cast::<MtrlHeader>(&self.data);

        let base_offset = size_of::<MtrlHeader>();
        let string_base = base_offset + ((header.texture_count + header.map_count + header.color_set_count) as usize) * size_of::<u32>();

        (0..header.texture_count)
            .map(|x| {
                let value = (&self.data[base_offset + (x as usize) * size_of::<u32>()..]).to_int_le::<u32>();
                let offset = value & 0xffff;
                let flag = value >> 16;

                let path = str::from_null_terminated_utf8(&self.data[string_base + offset as usize..]).unwrap();
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

    pub fn color_table(&self) -> &[u8] {
        let header = cast::<MtrlHeader>(&self.data);

        let offset = size_of::<MtrlHeader>()
            + ((header.texture_count + header.map_count + header.color_set_count) as usize) * size_of::<u32>()
            + header.strings_size as usize
            + size_of::<u32>();

        &self.data[offset..offset + header.color_table_size as usize]
    }

    pub fn shader_name(&self) -> &str {
        let header = cast::<MtrlHeader>(&self.data);
        let string_base = size_of::<MtrlHeader>() + ((header.texture_count + header.map_count + header.color_set_count) as usize) * size_of::<u32>();

        str::from_null_terminated_utf8(&self.data[string_base + header.shader_name_offset as usize..]).unwrap()
    }
}
