use nom::number::complete::le_u32;
use nom::{do_parse, named, tag};

pub struct LvbHeader {
    pub file_size: u32,
    pub header_size: u32,
}

impl LvbHeader {
    pub const SIZE: usize = 20;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"LVB1")   >>
            file_size:      le_u32          >>
            /* unk1: */     le_u32          >>
            /* magic: */    tag!(b"SCN1")   >>
            header_size:    le_u32          >>
            (Self {
                file_size,
                header_size
            })
        )
    );
}

pub struct LvbEntries {
    pub entry1_offset: u32,
    pub entry2_offset: u32,
    pub entry3_offset: u32,
    pub entry4_offset: u32,
    pub entry4_count: u32,
    pub entry5_offset: u32,
}

impl LvbEntries {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            entry1_offset:  le_u32  >>
            /* unk1: */     le_u32  >>
            /* unk2: */     le_u32  >>
            entry2_offset:  le_u32  >>
            entry3_offset:  le_u32  >>
            entry4_offset:  le_u32  >>
            entry4_count:   le_u32  >>
            /* unk3: */     le_u32  >>
            entry5_offset:  le_u32  >>
            (Self {
                entry1_offset,
                entry2_offset,
                entry3_offset,
                entry4_offset,
                entry4_count,
                entry5_offset,
            })
        )
    );
}
