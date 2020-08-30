use alloc::vec::Vec;
use core::mem::size_of;

use bytes::Bytes;

use sqpack::SqPackRawFile;
use util::{cast, round_up};

#[repr(C)]
struct ExtractedFileHeader {
    uncompressed_size: u32,
    header_size: u32,
    block_count: u32,
}

fn get_block_size(block: &[u8]) -> usize {
    let header = SqPackRawFile::get_block_header(block);

    if header.compressed_length >= 32000 {
        header.header_size as usize + header.uncompressed_length as usize
    } else {
        header.header_size as usize + header.compressed_length as usize
    }
}

pub trait ExtractedSqPackRawFile {
    fn from_extracted_file(data: Vec<u8>) -> Self;
    #[cfg(feature = "std")]
    fn into_extracted(self) -> Vec<u8>;
}

impl ExtractedSqPackRawFile for SqPackRawFile {
    fn from_extracted_file(data: Vec<u8>) -> Self {
        let data = Bytes::from(data);
        let file_header = cast::<ExtractedFileHeader>(&data);

        let header = data.slice(size_of::<ExtractedFileHeader>()..size_of::<ExtractedFileHeader>() + file_header.header_size as usize);

        let begin = size_of::<ExtractedFileHeader>() + file_header.header_size as usize;
        let blocks = (0..file_header.block_count)
            .scan(begin, |offset, _| {
                let block_size = get_block_size(&data[*offset..]);
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
    fn into_extracted(self) -> Vec<u8> {
        use core::iter;

        let mut result = Vec::with_capacity(self.uncompressed_size as usize + size_of::<ExtractedFileHeader>());
        result.extend(self.uncompressed_size.to_le_bytes().iter());
        result.extend((self.header.len() as u32).to_le_bytes().iter());
        result.extend((self.blocks.len() as u32).to_le_bytes().iter());

        for block in self.blocks {
            let block_size = get_block_size(&block);
            result.extend(&block[0..block_size]);

            let rounded_size = round_up(block_size, 4);
            result.extend(iter::repeat(0).take(rounded_size - block_size));
        }

        result
    }
}
