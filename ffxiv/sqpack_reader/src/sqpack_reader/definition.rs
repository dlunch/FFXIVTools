use nom::number::complete::{le_u16, le_u32};
use nom::{count, do_parse, named, take};

// TODO use nom-derive (https://github.com/rust-bakery/nom-derive)

pub struct SqPackHeader {
    pub header_length: u32,
    pub unk1: u32,
    pub file_type: u32,
}

impl SqPackHeader {
    pub const SIZE: usize = 24;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* signature: */    take!(12)   >>
            header_length:      le_u32      >>
            unk1:               le_u32      >>
            file_type:          le_u32      >>
            (Self {
                header_length,
                unk1,
                file_type,
            })
        )
    );
}

pub struct SqPackIndexSegment {
    pub offset: u32, // 0
    pub size: u32,   // 4
}

#[rustfmt::skip]
named!(sqpack_index_segment<SqPackIndexSegment>,
    do_parse!(
        offset:         le_u32       >>
        size:           le_u32       >>
        _hash:          take!(12)    >>
        _padding:       take!(52)    >>
        (SqPackIndexSegment {
            offset,
            size,
        })
    )
);

pub struct SqPackIndexHeader {
    pub header_length: u32,
    pub unk: u32,
    pub file_segment: SqPackIndexSegment,
    pub dat_count: u32,
    pub segment2: SqPackIndexSegment,
    pub segment3: SqPackIndexSegment,
    pub folder_segment: SqPackIndexSegment,
}

impl SqPackIndexHeader {
    pub const SIZE: usize = 300;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            header_length:      le_u32                  >>
            unk:                le_u32                  >>
            file_segment:       sqpack_index_segment    >>
            dat_count:          le_u32                  >>
            segment2:           sqpack_index_segment    >>
            segment3:           sqpack_index_segment    >>
            folder_segment:     sqpack_index_segment    >>
            (Self {
                header_length,
                unk,
                file_segment,
                dat_count,
                segment2,
                segment3,
                folder_segment,
            })
        )
    );
}

pub struct FileSegment {
    pub file_hash: u32,
    pub folder_hash: u32,
    pub data_offset: u32,
}

impl FileSegment {
    pub const SIZE: usize = 16;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            file_hash:      le_u32  >>
            folder_hash:    le_u32  >>
            data_offset:    le_u32  >>
            _padding:       le_u32  >>
            (Self {
                file_hash,
                folder_hash,
                data_offset
            })
        )
    );
}

/*
pub struct FileSegment2 {
    file_hash: u32,
    data_offset: u32,
}
*/

pub struct FolderSegment {
    pub folder_hash: u32,
    pub file_list_offset: u32,
    pub file_list_size: u32,
}

impl FolderSegment {
    pub const SIZE: usize = 16;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            folder_hash:        le_u32  >>
            file_list_offset:   le_u32  >>
            file_list_size:     le_u32  >>
            _padding:           le_u32  >>
            (Self {
                folder_hash,
                file_list_offset,
                file_list_size
            })
        )
    );
}

pub const FILE_TYPE_DEFAULT: u32 = 2;
pub const FILE_TYPE_MODEL: u32 = 3;
pub const FILE_TYPE_IMAGE: u32 = 4;

pub struct FileHeader {
    pub header_length: u32,
    pub file_type: u32,
    pub uncompressed_size: u32,
    pub frame_count: u32,
}

impl FileHeader {
    pub const SIZE: usize = 24;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            header_length:      le_u32  >>
            file_type:          le_u32  >>
            uncompressed_size:  le_u32  >>
            _unk1:              le_u32  >>
            _unk2:              le_u32  >>
            frame_count:        le_u32  >>
            (Self {
                header_length,
                file_type,
                uncompressed_size,
                frame_count
            })
        )
    );
}

pub struct DefaultFrameHeader {
    pub block_offset: u32,
    pub block_size: u16,
    pub uncompressed_size: u16,
}

impl DefaultFrameHeader {
    pub const SIZE: usize = 8;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            block_offset:       le_u32  >>
            block_size:         le_u16  >>
            uncompressed_size:  le_u16  >>
            (Self {
                block_offset,
                block_size,
                uncompressed_size,
            })
        )
    );
}

pub const MODEL_CHUNK_COUNT: usize = 11;

pub struct ModelFrameHeader {
    pub uncompressed_chunk_sizes: Vec<u32>,
    pub sizes: Vec<u32>,
    pub offsets: Vec<u32>,
    pub start_block_indices: Vec<u16>,
    pub block_counts: Vec<u16>,
    pub number_of_meshes: u16,
    pub number_of_materials: u16,
}

impl ModelFrameHeader {
    pub const SIZE: usize = 184;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            uncompressed_chunk_sizes:   count!(le_u32, MODEL_CHUNK_COUNT)   >>
            sizes:                      count!(le_u32, MODEL_CHUNK_COUNT)   >>
            offsets:                    count!(le_u32, MODEL_CHUNK_COUNT)   >>
            start_block_indices:        count!(le_u16, MODEL_CHUNK_COUNT)   >>
            block_counts:               count!(le_u16, MODEL_CHUNK_COUNT)   >>
            number_of_meshes:           le_u16                              >>
            number_of_materials:        le_u16                              >>
            _unk1:                      le_u16                              >>
            _unk2:                      le_u16                              >>
            (Self {
                uncompressed_chunk_sizes,
                sizes,
                offsets,
                start_block_indices,
                block_counts,
                number_of_meshes,
                number_of_materials
            })
        )
    );
}

pub struct ImageFrameHeader {
    pub block_offset: u32,
    pub block_size: u32,
    pub sizes_table_offset: u32,
    pub block_count: u32,
}

impl ImageFrameHeader {
    pub const SIZE: usize = 20;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            block_offset:       le_u32 >>
            block_size:         le_u32 >>
            _unk:               le_u32 >>
            sizes_table_offset: le_u32 >>
            block_count:        le_u32 >>
            (Self {
                block_offset,
                block_size,
                sizes_table_offset,
                block_count,
            })
        )
    );
}
