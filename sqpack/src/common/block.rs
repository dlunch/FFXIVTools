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
pub struct SqPackDataBlock {
    header: BlockHeader,
    data: Vec<u8>,
}

impl SqPackDataBlock {
    pub async fn new(file: &mut File, offset: u64) -> io::Result<SqPackDataBlock> {
        let header = read_and_parse!(file, offset, BlockHeader).await?;
        let length = if header.compressed_length >= 32000 {
            header.uncompressed_length
        } else {
            header.compressed_length
        };
        let data = file.read_to_vec(offset + header.header_size as u64, length as usize).await?;

        Ok(SqPackDataBlock { header, data })
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
