use std::fs::File;
use std::io;
use std::path::Path;

use super::definition::{BlockHeader, DefaultBlockHeader, FileHeader, FILE_TYPE_DEFAULT};
use super::ext::ReadExt;
use compression::prelude::DecodeExt;
use compression::prelude::Deflater;

struct SqPackDataBlock {
    header: BlockHeader,
    data: Vec<u8>,
}

pub struct SqPackData {
    file: File,
}

impl SqPackData {
    pub fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;

        Ok(Self { file })
    }

    pub fn read(&mut self, offset: u64) -> io::Result<Vec<u8>> {
        let file_header = read_and_parse!(self.file, offset, FileHeader);
        let blocks = self.read_blocks(offset, &file_header)?;

        Ok(Self::decode_blocks(blocks))
    }

    fn read_blocks(
        &mut self,
        base_offset: u64,
        file_header: &FileHeader,
    ) -> io::Result<Vec<SqPackDataBlock>> {
        let block_offsets = match file_header.file_type {
            FILE_TYPE_DEFAULT => self.read_block_offsets_default(base_offset, &file_header),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Incorrect header",
            )),
        }?;

        let file = &mut self.file;
        Ok(block_offsets
            .iter()
            .map(|x| {
                let header = read_and_parse!(file, *x, BlockHeader);
                let length = if header.compressed_length >= 32000 {
                    header.uncompressed_length
                } else {
                    header.compressed_length
                };
                let data = file.read_to_vec(x + header.header_size as u64, length as usize)?;

                Ok(SqPackDataBlock { header, data })
            })
            .collect::<io::Result<Vec<_>>>()?)
    }

    fn decode_blocks(mut blocks: Vec<SqPackDataBlock>) -> Vec<u8> {
        blocks
            .drain(..)
            .flat_map(|x| {
                if x.header.compressed_length >= 32000 {
                    x.data
                } else {
                    let mut data = x.data;
                    data.drain(..)
                        .decode(&mut Deflater::new())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap()
                }
            })
            .collect()
    }

    fn read_block_offsets_default(
        &mut self,
        base_offset: u64,
        file_header: &FileHeader,
    ) -> io::Result<Vec<u64>> {
        let block_headers = read_and_parse!(
            self.file,
            base_offset + FileHeader::SIZE as u64,
            file_header.block_count,
            DefaultBlockHeader
        );

        Ok(block_headers
            .iter()
            .map(|x| base_offset + file_header.header_length as u64 + x.offset as u64)
            .collect())
    }
}
