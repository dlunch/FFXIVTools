use nom::number::complete::le_u32;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result, Seek, SeekFrom};

use crate::package::Package;

#[allow(dead_code)]
struct SqPackHeader<'a> {
    signature: &'a [u8], // 0
    header_length: u32,  // 12
    unk1: u32,           // 16
    file_type: u32,      // 20
}
const SQPACK_HEADER_SIZE: usize = 24;

#[rustfmt::skip]
named!(parse_sqpack_header<SqPackHeader>,
    do_parse!(
        signature:      take!(12)   >>
        header_length:  le_u32      >>
        unk1:           le_u32      >>
        file_type:      le_u32      >>
        (SqPackHeader {
            signature,
            header_length,
            unk1,
            file_type,
        })
    )
);

#[allow(dead_code)]
struct SqPackIndexSegment<'a> {
    offset: u32,       // 0
    size: u32,         // 4
    hash: &'a [u8],    // 8
    padding: &'a [u8], // 20
}

#[rustfmt::skip]
named!(parse_sqpack_index_segment<SqPackIndexSegment>,
    do_parse!(
        offset:     le_u32       >>
        size:       le_u32       >>
        hash:       take!(12)    >>
        padding:    take!(52)    >>
        (SqPackIndexSegment {
            offset,
            size,
            hash,
            padding,
        })
    )
);

#[allow(dead_code)]
struct SqPackIndexSegmentHeader<'a> {
    header_length: u32,
    unk: u32,
    file_segment: SqPackIndexSegment<'a>,
    dat_count: u32,
    segment2: SqPackIndexSegment<'a>,
    segment3: SqPackIndexSegment<'a>,
    folder_segment: SqPackIndexSegment<'a>,
}
const SQPACK_INDEX_SEGMENT_HEADER_SIZE: usize = 300;

#[rustfmt::skip]
named!(parse_sqpack_index_segment_header<SqPackIndexSegmentHeader>,
    do_parse!(
        header_length:      le_u32                      >>
        unk:                le_u32                      >>
        file_segment:       parse_sqpack_index_segment  >>
        dat_count:          le_u32                      >>
        segment2:           parse_sqpack_index_segment  >>
        segment3:           parse_sqpack_index_segment  >>
        folder_segment:     parse_sqpack_index_segment  >>
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

#[derive(Default)]
pub struct SqPack {}

impl SqPack {
    pub fn mount(&self, path: &str) -> Result<()> {
        let index_path = format!("{}.index", path);
        let f = File::open(index_path)?;
        let mut r = BufReader::new(f);
        let header_length = SqPack::read_header_length(&mut r)?;

        r.seek(SeekFrom::Start(header_length.into()))?;
        let mut index_header_buf = [0u8; SQPACK_INDEX_SEGMENT_HEADER_SIZE];
        r.read_exact(&mut index_header_buf)?;
        let (_, index_header) = parse_sqpack_index_segment_header(&index_header_buf).unwrap();

        Ok(())
    }

    fn read_header_length<R: BufRead>(r: &mut R) -> Result<u32> {
        let mut header_buf = [0u8; SQPACK_HEADER_SIZE];
        r.read_exact(&mut header_buf)?;
        let (_, header) = parse_sqpack_header(&header_buf).unwrap();

        Ok(header.header_length)
    }
}

impl Package for SqPack {
    fn read_file(&self, filename: &str) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    #[test]
    fn test_read() {
        let pack = SqPack {};
        pack.mount("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ffxiv\\0a0000.win32")
            .unwrap();
    }
}
