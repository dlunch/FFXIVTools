use std::fs::File;
use std::io;
use std::path::Path;

use super::ext::ReadExt;
use super::parser::{BlockHeader, DefaultBlockHeader, FileHeader, FILE_TYPE_DEFAULT};

struct SqPackDataBlock {
    header: BlockHeader,
    data: Vec<u8>,
}

pub struct SqPackData {
    file: File,
}

impl SqPackData {
    pub fn new(path: &Path) -> io::Result<SqPackData> {
        let file = File::open(path)?;

        Ok(SqPackData { file })
    }

    pub fn read(&mut self, offset: usize) -> io::Result<Vec<u8>> {
        let file_header = read_and_parse!(self.file, offset, FileHeader);
        let block_offsets = self.read_block_offsets(offset, &file_header)?;

        Ok(Vec::new())
    }

    fn read_block_offsets(
        &mut self,
        base_offset: usize,
        file_header: &FileHeader,
    ) -> io::Result<Vec<usize>> {
        match file_header.file_type {
            FILE_TYPE_DEFAULT => self.read_blocks_default(base_offset, &file_header),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Incorrect header",
            )),
        }
    }

    fn read_blocks_default(
        &mut self,
        base_offset: usize,
        file_header: &FileHeader,
    ) -> io::Result<Vec<usize>> {
        let block_headers = read_and_parse!(
            self.file,
            base_offset + FileHeader::SIZE,
            file_header.block_count,
            DefaultBlockHeader
        );

        Ok(block_headers
            .iter()
            .map(|x| base_offset + file_header.header_length as usize + x.offset as usize)
            .collect())
    }
}
