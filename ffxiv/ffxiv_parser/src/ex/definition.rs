use nom::number::complete::{be_u16, be_u32};
use nom::{do_parse, named, tag};

pub struct ExhHeader {
    pub version: u16,
    pub row_size: u16,
    pub column_count: u16,
    pub page_count: u16,
    pub language_count: u16,
    pub row_type: u16,
    pub item_count: u32,
}

impl ExhHeader {
    pub const SIZE: usize = 32;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            /* magic: */    tag!(b"EXHF")   >>
            version:        be_u16          >>
            row_size:       be_u16          >>
            column_count:   be_u16          >>
            page_count:     be_u16          >>
            language_count: be_u16          >>
            /* unk1: */     be_u16          >>
            row_type:       be_u16          >>
            /* unk2: */     be_u16          >>
            item_count:     be_u32          >>
            /* unk3: */     be_u32          >>
            /* unk4: */     be_u32          >>
            (Self {
                version,
                row_size,
                column_count,
                page_count,
                language_count,
                row_type,
                item_count,
            })
        )
    );
}

pub struct ExhColumnDefinition {
    pub field_type: u16,
    pub offset: u16,
}

impl ExhColumnDefinition {
    pub const SIZE: usize = 4;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            field_type: be_u16  >>
            offset:     be_u16  >>
            (Self {
                field_type,
                offset,
            })
        )
    );
}

#[derive(Copy, Clone)]
pub struct ExhPage {
    pub start: u32,
    pub count: u32,
}

impl ExhPage {
    pub const SIZE: usize = 8;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            start:  be_u32  >>
            count:  be_u32  >>
            (Self {
                start,
                count,
            })
        )
    );
}
