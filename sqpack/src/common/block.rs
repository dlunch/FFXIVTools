use std::io;

use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};
use tokio::fs::File;

use super::util::ReadExt;

pub struct BlockHeader {
    pub header_size: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

impl BlockHeader {
    pub const SIZE: usize = 16;

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

// TODO move to util
fn round_up(num_to_round: u64, multiple: u64) -> u64 {
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

pub struct SqPackDataBlock {
    header: BlockHeader,
    data: Vec<u8>,
}

impl SqPackDataBlock {
    pub async fn new(file: &mut File, offset: u64) -> io::Result<SqPackDataBlock> {
        Ok(Self::read(file, offset).await?.0)
    }

    pub async fn with_compressed_data(file: &mut File, offset: u64, count: usize) -> io::Result<Vec<SqPackDataBlock>> {
        let mut result = Vec::with_capacity(count);

        let mut offset = offset;
        for _ in 0..count {
            let item = Self::read(file, offset).await?;
            result.push(item.0);

            offset += round_up(item.1, 4u64);
        }

        Ok(result)
    }

    pub async fn read(file: &mut File, offset: u64) -> io::Result<(SqPackDataBlock, u64)> {
        let header = read_and_parse!(file, offset, BlockHeader).await?;
        let length = if header.compressed_length >= 32000 {
            header.uncompressed_length
        } else {
            header.compressed_length
        };

        let header_size = header.header_size as u64;
        let data = file.read_to_vec(offset + header_size, length as usize).await?;

        Ok((SqPackDataBlock { header, data }, header_size + length as u64))
    }

    pub fn decode(mut self) -> impl Iterator<Item = u8> {
        if self.header.compressed_length >= 32000 {
            self.data.into_iter()
        } else {
            self.data
                .drain(..)
                .decode(&mut Deflater::new())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .into_iter()
        }
    }
}
