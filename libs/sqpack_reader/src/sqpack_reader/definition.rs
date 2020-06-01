#[repr(C)]
pub struct SqPackHeader {
    _signature: [u8; 12],
    pub header_length: u32,
    _unk: u32,
    pub file_type: u32,
}

#[repr(C)]
pub struct SqPackIndexHeader {
    pub header_length: u32,
    _unk: u32,
    pub file_segment: SqPackIndexSegment,
    pub dat_count: u32,
    pub segment2: SqPackIndexSegment,
    pub segment3: SqPackIndexSegment,
    pub folder_segment: SqPackIndexSegment,
}

#[repr(C)]
pub struct SqPackIndexSegment {
    pub offset: u32,
    pub size: u32,
    _hash: [u8; 20],
    _padding: [u8; 44],
}

#[derive(Clone)]
#[repr(C)]
pub struct FileSegment {
    pub file_hash: u32,
    pub folder_hash: u32,
    pub data_offset: u32,
    _padding: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct FolderSegment {
    pub folder_hash: u32,
    pub file_list_offset: u32,
    pub file_list_size: u32,
    _padding: u32,
}

#[repr(u32)]
pub enum FileType {
    Default = 2,
    Model = 3,
    Image = 4,
}

impl FileType {
    pub fn from_raw(raw: u32) -> Self {
        match raw {
            2 => FileType::Default,
            3 => FileType::Model,
            4 => FileType::Image,
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct FileHeader {
    pub header_length: u32,
    pub file_type: u32,
    pub uncompressed_size: u32,
    _unk1: u32,
    _unk2: u32,
    pub frame_count: u32,
}

#[repr(C)]
pub struct DefaultFrameInfo {
    pub block_offset: u32,
    pub block_size: u16,
    pub uncompressed_size: u16,
}

pub const MODEL_CHUNK_COUNT: usize = 11;

#[repr(C)]
pub struct ModelFrameInfo {
    pub uncompressed_chunk_sizes: [u32; MODEL_CHUNK_COUNT],
    pub sizes: [u32; MODEL_CHUNK_COUNT],
    pub offsets: [u32; MODEL_CHUNK_COUNT],
    pub start_block_indices: [u16; MODEL_CHUNK_COUNT],
    pub block_counts: [u16; MODEL_CHUNK_COUNT],
    pub number_of_meshes: u16,
    pub number_of_materials: u16,
    _unk1: u16,
    _unk2: u16,
}

#[repr(C)]
pub struct ImageFrameInfo {
    pub block_offset: u32,
    pub block_size: u32,
    _unk: u32,
    pub sizes_table_offset: u32,
    pub block_count: u32,
}
