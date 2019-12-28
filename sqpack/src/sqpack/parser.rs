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
        /* hash: */     take!(12)    >>
        /* padding: */  take!(52)    >>
        (SqPackIndexSegment {
            offset,
            size,
        })
    )
);

pub struct SqPackIndexSegmentHeader {
    pub header_length: u32,
    pub unk: u32,
    pub file_segment: SqPackIndexSegment,
    pub dat_count: u32,
    pub segment2: SqPackIndexSegment,
    pub segment3: SqPackIndexSegment,
    pub folder_segment: SqPackIndexSegment,
}

impl SqPackIndexSegmentHeader {
    pub const SIZE: usize = 300;

    #[rustfmt::skip]
    named!(pub parse<SqPackIndexSegmentHeader>,
        do_parse!(
            header_length:      le_u32                  >>
            unk:                le_u32                  >>
            file_segment:       sqpack_index_segment    >>
            dat_count:          le_u32                  >>
            segment2:           sqpack_index_segment    >>
            segment3:           sqpack_index_segment    >>
            folder_segment:     sqpack_index_segment    >>
            (SqPackIndexSegmentHeader {
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
    name_hash: u32,
    folder_hash: u32,
    data_offset: u32,
    padding: u32,
}

pub struct FileSegment2 {
    name_hash: u32,
    data_offset: u32,
}

pub struct FolderSegment {
    name_hash: u32,
    file_list_offset: u32,
    file_list_size: u32,
    padding: u32,
}

macro_rules! parse {
    ($reader: ident, $type: ty) => {{
        parse!($reader, $type, 0u32)
    }};

    ($reader: ident, $type: ty, $offset: expr) => {{
        let mut buf = [0u8; <$type>::SIZE];
        $reader.seek(SeekFrom::Start($offset as u64))?;
        $reader.read_exact(&mut buf)?;
        let (_, result) = <$type>::parse(&buf).unwrap();

        result
    }};
}
