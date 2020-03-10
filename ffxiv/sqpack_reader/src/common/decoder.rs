use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};

use util::{parse, round_up};

struct BlockHeader {
    pub header_size: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

impl BlockHeader {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            header_size:            le_u32  >>
            /* unk: */              le_u32  >>
            compressed_length:      le_u32  >>
            uncompressed_length:    le_u32  >>
            (Self {
                header_size,
                compressed_length,
                uncompressed_length,
            })
        )
    );
}

struct CompressedFileHeader {
    uncompressed_size: u32,
    additional_header_size: u32,
    block_count: u32,
}

impl CompressedFileHeader {
    const SIZE: usize = 12;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            uncompressed_size:      le_u32  >>
            additional_header_size: le_u32  >>
            block_count:            le_u32  >>
            (Self {
                uncompressed_size,
                additional_header_size,
                block_count,
            })
        )
    );
}

pub fn decode_block_into(result: &mut Vec<u8>, data: &[u8]) -> usize {
    let header = parse!(&data, BlockHeader);

    if header.compressed_length >= 32000 {
        let end = header.header_size as usize + header.uncompressed_length as usize;
        result.extend(&data[header.header_size as usize..end]);

        end
    } else {
        let end = header.header_size as usize + header.compressed_length as usize;
        let data = &data[header.header_size as usize..end];

        result.extend(data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap());

        end
    }
}

pub fn decode_compressed_data(data: &[u8]) -> Vec<u8> {
    let header = parse!(&data, CompressedFileHeader);

    let mut result = Vec::with_capacity(header.uncompressed_size as usize);

    let additional_header = &data[CompressedFileHeader::SIZE..CompressedFileHeader::SIZE + header.additional_header_size as usize];
    result.extend(additional_header);

    let mut offset = CompressedFileHeader::SIZE + header.additional_header_size as usize;
    for _ in 0..header.block_count {
        let consumed = decode_block_into(&mut result, &data[offset..]);

        offset += round_up(consumed, 4usize);
    }

    result
}
