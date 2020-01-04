use nom::number::complete::le_u32;

pub struct SqPackHeader {
    pub header_length: u32,
    pub unk1: u32,
    pub file_type: u32,
}

impl SqPackHeader {
    pub const SIZE: usize = 24;

    #[rustfmt::skip]
    named!(pub parse<SqPackHeader>,
        do_parse!(
            /* signature: */    take!(12)   >>
            header_length:      le_u32      >>
            unk1:               le_u32      >>
            file_type:          le_u32      >>
            (SqPackHeader {
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
    named!(pub parse<SqPackIndexHeader>,
        do_parse!(
            header_length:      le_u32                  >>
            unk:                le_u32                  >>
            file_segment:       sqpack_index_segment    >>
            dat_count:          le_u32                  >>
            segment2:           sqpack_index_segment    >>
            segment3:           sqpack_index_segment    >>
            folder_segment:     sqpack_index_segment    >>
            (SqPackIndexHeader {
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
    named!(pub parse<FileSegment>,
        do_parse!(
            file_hash:      le_u32  >>
            folder_hash:    le_u32  >>
            data_offset:    le_u32  >>
            _padding:       le_u32  >>
            (FileSegment {
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
    named!(pub parse<FolderSegment>,
        do_parse!(
            folder_hash:        le_u32  >>
            file_list_offset:   le_u32  >>
            file_list_size:     le_u32  >>
            _padding:           le_u32  >>
            (FolderSegment {
                folder_hash,
                file_list_offset,
                file_list_size
            })
        )
    );
}
