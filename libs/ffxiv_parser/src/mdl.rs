use core::mem::size_of;

use sqpack_reader::{Package, Result};
use util::{cast, cast_array, SliceByteOrderExt, StrExt};

#[repr(C)]
struct MdlHeader {
    _unk1: u32,
    mesh_count: u16,
    attribute_count: u16,
    part_count: u16,
    material_count: u16,
    bone_count: u16,
    bone_list_count: u16,
    shp_count: u16,
    _unk_count1: u16,
    _unk_count2: u16,
    _unk2: u16,
    unk_count3: u16,
    unk_count4: u8,
    _unk3: u8,
    _unk4: [u16; 5],
    unk_count5: u16,
    _unk5: [u16; 8],
}

#[repr(C)]
struct MeshPart {
    index_offset: u32,
    index_count: u32,
    attributes: u32,
    bone_offset: u16,
    bone_count: u16,
}

#[repr(C)]
struct ModelHeader {
    mesh_offset: u16,
    mesh_count: u16,
    _unk1: [u16; 20],
    vertex_buffer_size: u32,
    index_buffer_size: u32,
    buffer_data_offset: u32,
    index_data_offset: u32,
}

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

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BufferItemUsage {
    Position = 0,
    BoneWeight = 1,
    BoneIndex = 2,
    Normal = 3,
    TexCoord = 4,
    Tangent = 5,
    BiTangent = 6,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BufferItemChunk {
    buffer_items: [BufferItem; 17],
}

impl BufferItemChunk {
    pub fn items(&self) -> impl Iterator<Item = &BufferItem> {
        self.buffer_items.iter().take_while(|x| x.buffer != 255)
    }
}

#[repr(C)]
pub struct MeshInfo {
    pub vertex_count: u32,
    pub index_count: u32,
    pub material_index: u16,
    pub part_offset: u16,
    pub part_count: u16,
    pub bone_index: u16,
    pub index_offset: u32,
    pub buffer_offsets: [u32; 3],
    pub strides: [u8; 3],
    pub buffer_count: u8,
}

pub struct Mesh<'a> {
    pub mesh_info: &'a MeshInfo,
    pub buffers: Vec<&'a [u8]>,
    pub indices: &'a [u8],
}

pub struct Mdl {
    data: Vec<u8>,
    string_block_offset: usize,
    mdl_header_offset: usize,
    model_header_offset: usize,
    mesh_info_offset: usize,
    attribute_info_offset: usize,
}

impl Mdl {
    const LOD_COUNT: usize = 3;

    pub async fn new<T: AsRef<str>>(package: &dyn Package, path: T) -> Result<Self> {
        let data = package.read_file(path.as_ref()).await?;

        let mesh_count = (&data[..]).to_int_le::<u16>() as usize;
        let mut cursor = 0x44 + size_of::<BufferItemChunk>() * mesh_count;

        let string_block_size = (&data[cursor + 4..]).to_int_le::<u32>() as usize;
        let string_block_offset = cursor + 8;
        cursor += string_block_size + 8;

        let mdl_header_offset = cursor;
        let mdl_header = cast::<MdlHeader>(&data[cursor..]);
        cursor += size_of::<MdlHeader>() + mdl_header.unk_count3 as usize * 0x20;

        let model_header_offset = cursor;
        let model_headers = &cast_array::<ModelHeader>(&data[cursor..])[..Self::LOD_COUNT];
        cursor += size_of::<ModelHeader>() * Self::LOD_COUNT;

        let mesh_info_offset = cursor;
        let mesh_info_count = model_headers.iter().map(|x| x.mesh_count as usize).sum::<usize>();
        let mesh_infos = &cast_array::<MeshInfo>(&data[cursor..])[..mesh_info_count];
        cursor += mesh_infos.len() * size_of::<MeshInfo>();
        let attribute_info_offset = cursor;

        Ok(Self {
            data,
            string_block_offset,
            mdl_header_offset,
            model_header_offset,
            mesh_info_offset,
            attribute_info_offset,
        })
    }

    pub fn buffer_items(&self, lod: usize) -> impl Iterator<Item = &BufferItemChunk> {
        let model_headers = &cast_array::<ModelHeader>(&self.data[self.model_header_offset..])[..Self::LOD_COUNT];
        let model_header = &model_headers[lod];

        const BUFFER_ITEM_OFFSET: usize = 0x44;
        let items_chunks = cast_array::<BufferItemChunk>(&self.data[BUFFER_ITEM_OFFSET..]);
        items_chunks
            .iter()
            .skip(model_header.mesh_offset as usize)
            .take(model_header.mesh_count as usize)
    }

    pub fn meshes<'a>(&'a self, lod: usize) -> impl Iterator<Item = Mesh> + 'a {
        let model_headers = &cast_array::<ModelHeader>(&self.data[self.model_header_offset..])[..Self::LOD_COUNT];
        let mesh_info_count = model_headers.iter().map(|x| x.mesh_count as usize).sum::<usize>();
        let mesh_infos = &cast_array::<MeshInfo>(&self.data[self.mesh_info_offset..])[..mesh_info_count];

        let model_header = &model_headers[lod];
        (0..model_header.mesh_count).map(move |mesh_index| {
            let mesh_info = &mesh_infos[model_header.mesh_offset as usize + mesh_index as usize];

            let buffers = (0..mesh_info.buffer_count)
                .map(|buffer_index| {
                    let buffer_begin = model_header.buffer_data_offset as usize + mesh_info.buffer_offsets[buffer_index as usize] as usize;
                    let buffer_end = buffer_begin + (mesh_info.vertex_count as usize) * (mesh_info.strides[buffer_index as usize] as usize);

                    &self.data[buffer_begin..buffer_end]
                })
                .collect::<Vec<_>>();

            let index_begin = model_header.index_data_offset as usize + (mesh_info.index_offset as usize) * size_of::<u16>();
            let index_end = index_begin + (mesh_info.index_count as usize) * size_of::<u16>();

            let indices = &self.data[index_begin..index_end];
            Mesh { mesh_info, buffers, indices }
        })
    }

    pub fn material_paths<'a>(&'a self) -> impl Iterator<Item = &str> + 'a {
        let mdl_header = cast::<MdlHeader>(&self.data[self.mdl_header_offset..]);
        let mut cursor = self.attribute_info_offset;

        cursor += (mdl_header.attribute_count as usize) * size_of::<u32>();
        cursor += (mdl_header.unk_count4 as usize) * 20;
        cursor += (mdl_header.part_count as usize) * size_of::<MeshPart>();
        cursor += (mdl_header.unk_count5 as usize) * 12;

        (0..mdl_header.material_count).map(move |x| {
            let string_offset = (&self.data[cursor + (x as usize) * size_of::<u32>()..]).to_int_le::<u32>() as usize;

            str::from_null_terminated_utf8(&self.data[self.string_block_offset + string_offset..]).unwrap()
        })
    }
}
