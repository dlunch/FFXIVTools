use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};

use super::MODEL_HEADER_SIZE;

pub struct BlockHeader {
    pub header_size: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

impl BlockHeader {
    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            header_size:            le_u32  >>
            _unk:                   le_u32  >>
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

enum DecodedResultData<'a> {
    Slice(&'a [u8]),
    Data(Vec<u8>),
}

pub struct DecodedResult<'a> {
    pub consumed: usize,
    data: DecodedResultData<'a>,
}

impl<'a> DecodedResult<'a> {
    pub fn with_data_slice(consumed: usize, data_slice: &'a [u8]) -> Self {
        Self {
            consumed,
            data: DecodedResultData::Slice(data_slice),
        }
    }

    pub fn with_data(consumed: usize, data: Vec<u8>) -> Self {
        Self {
            consumed,
            data: DecodedResultData::Data(data),
        }
    }

    pub fn data(&self) -> &[u8] {
        match self.data {
            DecodedResultData::Slice(x) => x,
            DecodedResultData::Data(ref x) => x,
        }
    }
}

pub fn decode_block(data: &[u8]) -> DecodedResult {
    let header = BlockHeader::parse(data).unwrap().1;

    if header.compressed_length >= 32000 {
        let end = header.header_size as usize + header.uncompressed_length as usize;
        let data = &data[header.header_size as usize..end];

        DecodedResult::with_data_slice(end, data)
    } else {
        let end = header.header_size as usize + header.compressed_length as usize;
        let data = &data[header.header_size as usize..end];

        let decoded = data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap();

        DecodedResult::with_data(end, decoded)
    }
}

// TODO move to util
fn round_up(num_to_round: usize, multiple: usize) -> usize {
    if multiple == 0 {
        return num_to_round;
    }

    let remainder = num_to_round % multiple;
    if remainder == 0 {
        num_to_round
    } else {
        num_to_round + multiple - remainder
    }
}

pub fn decode_compressed_data(data: Vec<u8>) -> Vec<u8> {
    const FILE_HEADER_SIZE: usize = 12;

    let mut reader = Cursor::new(&data);

    let uncompressed_size = reader.read_u32::<LittleEndian>().unwrap();
    let additional_header_size = reader.read_u32::<LittleEndian>().unwrap();
    let block_count = reader.read_u32::<LittleEndian>().unwrap();

    let mut additional_header = data[FILE_HEADER_SIZE as usize..FILE_HEADER_SIZE as usize + additional_header_size as usize].to_vec();
    if additional_header_size == 4 {
        additional_header.extend(vec![0; MODEL_HEADER_SIZE])
    }
    let mut result = Vec::with_capacity(uncompressed_size as usize);

    result.extend(additional_header);

    let mut offset = FILE_HEADER_SIZE + additional_header_size as usize;
    for _ in 0..block_count {
        let decoded = decode_block(&data[offset..]);
        result.extend(decoded.data());

        offset += round_up(decoded.consumed, 4usize);
    }

    result
}
