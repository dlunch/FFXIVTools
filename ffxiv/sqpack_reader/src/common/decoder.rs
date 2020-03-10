use bytes::{Buf, Bytes};
use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};

use super::util::round_up;

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

pub fn decode_block(data: Bytes) -> (usize, Bytes) {
    let header = BlockHeader::parse(&data).unwrap().1;

    if header.compressed_length >= 32000 {
        let end = header.header_size as usize + header.uncompressed_length as usize;
        let data = data.slice(header.header_size as usize..end);

        (end, data)
    } else {
        let end = header.header_size as usize + header.compressed_length as usize;
        let data = &data[header.header_size as usize..end];

        let decoded = data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap();

        (end, Bytes::from(decoded))
    }
}

pub fn decode_compressed_data(data: Bytes) -> Vec<u8> {
    const FILE_HEADER_SIZE: usize = 12;

    let mut header = data.slice(0..FILE_HEADER_SIZE);
    let uncompressed_size = header.get_u32_le();
    let additional_header_size = header.get_u32_le();
    let block_count = header.get_u32_le();

    let mut result = Vec::with_capacity(uncompressed_size as usize);

    let additional_header = data.slice(FILE_HEADER_SIZE as usize..FILE_HEADER_SIZE as usize + additional_header_size as usize);
    result.extend(additional_header);

    let mut offset = FILE_HEADER_SIZE + additional_header_size as usize;
    for _ in 0..block_count {
        let (consumed, decoded) = decode_block(data.slice(offset..));
        result.extend(decoded);

        offset += round_up(consumed, 4usize);
    }

    result
}
