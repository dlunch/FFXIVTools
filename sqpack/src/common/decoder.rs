use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use super::{decode_block_into, MODEL_HEADER_SIZE};

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
        let consumed = decode_block_into(&data[offset..], &mut result);

        offset += round_up(consumed, 4usize);
    }

    result
}
