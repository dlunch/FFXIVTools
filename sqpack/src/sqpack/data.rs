use byteorder::{LittleEndian, WriteBytesExt};
use nom::number::complete::le_u16;
use std::fs::File;
use std::io;
use std::path::Path;

use super::definition::{BlockHeader, DefaultBlockHeader, FileHeader, ModelBlockHeader, FILE_TYPE_DEFAULT, FILE_TYPE_MODEL};
use super::ext::ReadExt;
use compression::prelude::DecodeExt;
use compression::prelude::Deflater;

struct SqPackDataBlock {
    header: BlockHeader,
    data: Vec<u8>,
}

struct SqPackRawFile {
    additional_header: Vec<u8>,
    blocks: Vec<SqPackDataBlock>,
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
        let raw_file = self.read_raw(offset, file_header)?;

        Ok(Self::decode_raw(raw_file))
    }

    fn decode_raw(mut raw_file: SqPackRawFile) -> Vec<u8> {
        raw_file
            .additional_header
            .into_iter()
            .chain(raw_file.blocks.drain(..).flat_map(|x| {
                if x.header.compressed_length >= 32000 {
                    x.data
                } else {
                    let mut data = x.data;
                    data.drain(..).decode(&mut Deflater::new()).collect::<Result<Vec<_>, _>>().unwrap()
                }
            }))
            .collect()
    }

    fn read_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        match file_header.file_type {
            FILE_TYPE_DEFAULT => self.read_default_raw(base_offset, file_header),
            FILE_TYPE_MODEL => self.read_model_raw(base_offset, file_header),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect header")),
        }
    }

    fn read_blocks(&mut self, block_offsets: impl Iterator<Item = u64>) -> io::Result<Vec<SqPackDataBlock>> {
        let file = &mut self.file;
        Ok(block_offsets
            .map(|x| {
                let header = read_and_parse!(file, x, BlockHeader);
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

    fn read_default_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let block_headers = read_and_parse!(
            self.file,
            base_offset + FileHeader::SIZE as u64,
            file_header.block_count,
            DefaultBlockHeader
        );

        let block_offsets = block_headers
            .iter()
            .map(|x| base_offset + file_header.header_length as u64 + x.offset as u64);

        Ok(SqPackRawFile {
            additional_header: Vec::new(),
            blocks: self.read_blocks(block_offsets)?,
        })
    }

    named_args!(parse_block_sizes(count: usize)<Vec<u16>>, count!(le_u16, count));

    fn read_model_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let block_header = read_and_parse!(self.file, base_offset + FileHeader::SIZE as u64, ModelBlockHeader);

        let total_block_count = block_header.block_counts.iter().sum::<u16>() as usize;
        let block_size_data = self.file.read_to_vec(
            base_offset + FileHeader::SIZE as u64 + ModelBlockHeader::SIZE as u64,
            total_block_count as usize * std::mem::size_of::<u16>(),
        )?;
        let (_, block_sizes) = Self::parse_block_sizes(&block_size_data, total_block_count).unwrap();

        let block_size_sums = block_sizes.iter().scan(0usize, |acc, &x| {
            *acc += x as usize;
            Some(*acc)
        });

        let block_raw_offsets = (0..1).chain(block_size_sums.take(total_block_count - 1));
        let block_offsets = block_raw_offsets.map(|x| base_offset + file_header.header_length as u64 + block_header.offsets[0] as u64 + x as u64);
        Ok(SqPackRawFile {
            additional_header: Self::serialize_model_header(&block_header),
            blocks: self.read_blocks(block_offsets)?,
        })
    }

    fn serialize_model_header(block_header: &ModelBlockHeader) -> Vec<u8> {
        let mut result = Vec::with_capacity(0x44);

        result.write_u16::<LittleEndian>(block_header.number_of_meshes).unwrap();
        result.write_u16::<LittleEndian>(block_header.number_of_materials).unwrap();
        result.resize(0x44, 0);

        result
    }
}
