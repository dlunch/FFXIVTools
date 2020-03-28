use std::iter;

use compression::prelude::DecodeExt;
use compression::prelude::Deflater;
use nom::number::complete::le_u32;
use nom::{do_parse, named};

use bytes::Bytes;

use util::{parse, round_up};

struct BlockHeader {
    pub header_size: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

impl BlockHeader {
    const SIZE: usize = 16;

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
    header_size: u32,
    block_count: u32,
}

impl CompressedFileHeader {
    const SIZE: usize = 12;

    #[rustfmt::skip]
    named!(pub parse<Self>,
        do_parse!(
            uncompressed_size:  le_u32  >>
            header_size:        le_u32  >>
            block_count:        le_u32  >>
            (Self {
                uncompressed_size,
                header_size,
                block_count,
            })
        )
    );
}

pub struct SqPackRawFile {
    uncompressed_size: u32,
    header: Bytes,
    blocks: Vec<Bytes>,
}

impl SqPackRawFile {
    pub fn from_compressed_file(data: Vec<u8>) -> Self {
        let data = Bytes::from(data);
        let file_header = parse!(&data, CompressedFileHeader);

        let header = data.slice(CompressedFileHeader::SIZE..CompressedFileHeader::SIZE + file_header.header_size as usize);
        let mut blocks = Vec::with_capacity(file_header.block_count as usize);

        let mut offset = CompressedFileHeader::SIZE + file_header.header_size as usize;
        for _ in 0..file_header.block_count {
            let block_size = Self::get_block_size(&data[offset..offset + BlockHeader::SIZE]);
            let block = data.slice(offset..offset + block_size);
            blocks.push(block);

            offset += round_up(block_size, 4usize);
        }

        Self {
            uncompressed_size: file_header.uncompressed_size,
            header,
            blocks,
        }
    }

    pub fn from_blocks(uncompressed_size: u32, header: Vec<u8>, data: Vec<Vec<u8>>) -> Self {
        let mut blocks = Vec::with_capacity(data.len());
        for data in data {
            blocks.push(Bytes::from(data));
        }

        Self {
            uncompressed_size,
            header: Bytes::from(header),
            blocks,
        }
    }

    pub fn from_contiguous_block(uncompressed_size: u32, header: Vec<u8>, block_data: Vec<u8>, block_sizes: Vec<u16>) -> Self {
        let mut blocks = Vec::new();
        let mut offset = 0usize;

        let data = Bytes::from(block_data);
        for block_size in block_sizes {
            blocks.push(data.slice(offset..));

            offset += block_size as usize;
        }

        Self {
            uncompressed_size,
            header: Bytes::from(header),
            blocks,
        }
    }

    pub fn from_contiguous_blocks(uncompressed_size: u32, header: Vec<u8>, contiguous_blocks: Vec<(Vec<u8>, Vec<u16>)>) -> Self {
        let mut blocks = Vec::with_capacity(
            contiguous_blocks
                .iter()
                .map(|(_, block_sizes)| block_sizes.iter().map(|&x| x as usize).sum::<usize>())
                .sum(),
        );

        for (block_data, block_sizes) in contiguous_blocks {
            let data = Bytes::from(block_data);
            let mut offset = 0usize;

            for block_size in block_sizes {
                blocks.push(data.slice(offset..));
                offset += block_size as usize;
            }
        }

        Self {
            uncompressed_size,
            header: Bytes::from(header),
            blocks,
        }
    }

    pub fn into_decoded(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.uncompressed_size as usize + self.header.len());
        result.extend(self.header);

        for block in &self.blocks {
            Self::decode_block_into(block, &mut result);
        }

        result
    }

    pub fn into_compressed(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.uncompressed_size as usize + CompressedFileHeader::SIZE);
        result.extend(&self.uncompressed_size.to_le_bytes());
        result.extend(&(self.header.len() as u32).to_le_bytes());
        result.extend(&(self.blocks.len() as u32).to_le_bytes());

        for block in self.blocks {
            let block_size = Self::get_block_size(&block);
            result.extend(&block[0..block_size]);

            let rounded_size = round_up(block_size, 4);
            result.extend(iter::repeat(0).take(rounded_size - block_size));
        }

        result
    }

    fn get_block_size(block: &[u8]) -> usize {
        let header = parse!(&block, BlockHeader);

        if header.compressed_length >= 32000 {
            header.header_size as usize + header.uncompressed_length as usize
        } else {
            header.header_size as usize + header.compressed_length as usize
        }
    }

    fn decode_block_into(block: &[u8], result: &mut Vec<u8>) {
        let header = parse!(&block, BlockHeader);

        if header.compressed_length >= 32000 {
            result.extend(&block[header.header_size as usize..header.header_size as usize + header.uncompressed_length as usize]);
        } else {
            let data = &block[header.header_size as usize..header.header_size as usize + header.compressed_length as usize];

            result.extend(data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap());
        }
    }
}
