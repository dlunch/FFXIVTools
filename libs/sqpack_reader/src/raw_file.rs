use alloc::vec::Vec;
use core::mem::size_of;

use compression::prelude::DecodeExt;
use compression::prelude::Deflater;

use bytes::{Bytes, BytesMut};

use util::{cast, round_up};

#[repr(C)]
struct BlockHeader {
    pub header_size: u32,
    _unk: u32,
    pub compressed_length: u32, // 32000 if not compressed
    pub uncompressed_length: u32,
}

#[repr(C)]
struct CompressedFileHeader {
    uncompressed_size: u32,
    header_size: u32,
    block_count: u32,
}

pub struct SqPackRawFile {
    uncompressed_size: u32,
    header: Bytes,
    blocks: Vec<Bytes>,
}

impl SqPackRawFile {
    pub fn from_compressed_file(data: Bytes) -> Self {
        let file_header = cast::<CompressedFileHeader>(&data);

        let header = data.slice(size_of::<CompressedFileHeader>()..size_of::<CompressedFileHeader>() + file_header.header_size as usize);

        let begin = size_of::<CompressedFileHeader>() + file_header.header_size as usize;
        let blocks = (0..file_header.block_count)
            .scan(begin, |offset, _| {
                let block_size = Self::get_block_size(&data[*offset..*offset + size_of::<BlockHeader>()]);
                let block = data.slice(*offset..*offset + block_size);

                *offset += round_up(block_size, 4usize);

                Some(block)
            })
            .collect::<Vec<_>>();

        Self {
            uncompressed_size: file_header.uncompressed_size,
            header,
            blocks,
        }
    }

    #[cfg(feature = "std")]
    pub fn from_blocks(uncompressed_size: u32, header: Bytes, blocks: Vec<Bytes>) -> Self {
        Self {
            uncompressed_size,
            header,
            blocks,
        }
    }

    pub fn into_decoded(self) -> Bytes {
        let mut result = BytesMut::with_capacity(self.uncompressed_size as usize + self.header.len());
        result.extend(self.header);

        for block in self.blocks {
            Self::decode_block_into(&block, &mut result);
        }

        result.freeze()
    }

    #[cfg(feature = "std")]
    pub fn into_compressed(self) -> Bytes {
        use bytes::BufMut;
        use core::iter;

        let mut result = BytesMut::with_capacity(self.uncompressed_size as usize + size_of::<CompressedFileHeader>());
        result.put_u32_le(self.uncompressed_size);
        result.put_u32_le(self.header.len() as u32);
        result.put_u32_le(self.blocks.len() as u32);

        for block in self.blocks {
            let block_size = Self::get_block_size(&block);
            result.extend(&block[0..block_size]);

            let rounded_size = round_up(block_size, 4);
            result.extend(iter::repeat(0).take(rounded_size - block_size));
        }

        result.freeze()
    }

    fn get_block_size(block: &[u8]) -> usize {
        let header = cast::<BlockHeader>(&block);

        if header.compressed_length >= 32000 {
            header.header_size as usize + header.uncompressed_length as usize
        } else {
            header.header_size as usize + header.compressed_length as usize
        }
    }

    fn decode_block_into(block: &[u8], result: &mut BytesMut) {
        let header = cast::<BlockHeader>(&block);

        if header.compressed_length >= 32000 {
            result.extend(&block[header.header_size as usize..header.header_size as usize + header.uncompressed_length as usize]);
        } else {
            let data = &block[header.header_size as usize..header.header_size as usize + header.compressed_length as usize];

            result.extend(data.iter().cloned().decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap());
        }
    }
}
