use byteorder::{LittleEndian, WriteBytesExt};
use nom::number::complete::le_u16;
use nom::{count, named_args};
use std::io;
use std::path::Path;
use tokio::fs::File;

use super::definition::{
    BlockHeader, DefaultFrameHeader, FileHeader, ImageFrameHeader, ModelFrameHeader, FILE_TYPE_DEFAULT, FILE_TYPE_IMAGE, FILE_TYPE_MODEL,
};
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
    pub async fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path).await?;

        Ok(Self { file })
    }

    pub async fn read(&mut self, offset: u64) -> io::Result<Vec<u8>> {
        let file_header = read_and_parse!(self.file, offset, FileHeader).await?;
        let raw_file = self.read_raw(offset, file_header).await?;

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

    async fn read_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        match file_header.file_type {
            FILE_TYPE_DEFAULT => Ok(self.read_default_raw(base_offset, file_header).await?),
            FILE_TYPE_MODEL => Ok(self.read_model_raw(base_offset, file_header).await?),
            FILE_TYPE_IMAGE => Ok(self.read_image_raw(base_offset, file_header).await?),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect header")),
        }
    }

    async fn read_blocks(&mut self, block_offsets: impl Iterator<Item = u64>) -> io::Result<Vec<SqPackDataBlock>> {
        let file = &mut self.file;
        let mut result = Vec::with_capacity(block_offsets.size_hint().0);

        for block_offset in block_offsets {
            let header = read_and_parse!(file, block_offset, BlockHeader).await?;
            let length = if header.compressed_length >= 32000 {
                header.uncompressed_length
            } else {
                header.compressed_length
            };
            let data = file.read_to_vec(block_offset + header.header_size as u64, length as usize).await?;

            result.push(SqPackDataBlock { header, data })
        }

        Ok(result)
    }

    async fn read_default_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_headers = read_and_parse!(
            self.file,
            base_offset + FileHeader::SIZE as u64,
            file_header.frame_count,
            DefaultFrameHeader
        )
        .await?;

        let block_offsets = frame_headers
            .iter()
            .map(|x| base_offset + file_header.header_length as u64 + x.block_offset as u64);

        Ok(SqPackRawFile {
            additional_header: Vec::new(),
            blocks: self.read_blocks(block_offsets).await?,
        })
    }

    async fn read_block_sizes(&mut self, offset: u64, count: usize) -> io::Result<Vec<u16>> {
        let block_size_data = self.file.read_to_vec(offset, count * std::mem::size_of::<u16>()).await?;

        named_args!(parse_block_sizes(count: usize)<Vec<u16>>, count!(le_u16, count));
        let (_, block_sizes) = parse_block_sizes(&block_size_data, count).unwrap();

        Ok(block_sizes)
    }

    fn block_sizes_to_offset<'a>(sizes: &'a [u16], base_offset: u64) -> impl Iterator<Item = u64> + 'a {
        let size_sums = sizes.iter().scan(0usize, |acc, &x| {
            *acc += x as usize;
            Some(*acc)
        });

        let raw_offsets = (0..1).chain(size_sums.take(sizes.len() - 1));
        raw_offsets.map(move |x| base_offset + x as u64)
    }

    async fn read_model_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_header = read_and_parse!(self.file, base_offset + FileHeader::SIZE as u64, ModelFrameHeader).await?;

        let total_block_count = frame_header.block_counts.iter().sum::<u16>() as usize;
        let sizes_offset = base_offset + FileHeader::SIZE as u64 + ModelFrameHeader::SIZE as u64;
        let block_sizes = self.read_block_sizes(sizes_offset, total_block_count).await?;
        let block_offsets = Self::block_sizes_to_offset(
            &block_sizes,
            base_offset + file_header.header_length as u64 + frame_header.offsets[0] as u64,
        );

        Ok(SqPackRawFile {
            additional_header: Self::serialize_model_header(&frame_header),
            blocks: self.read_blocks(block_offsets).await?,
        })
    }

    fn serialize_model_header(frame_header: &ModelFrameHeader) -> Vec<u8> {
        let mut result = Vec::with_capacity(0x44);

        result.write_u16::<LittleEndian>(frame_header.number_of_meshes).unwrap();
        result.write_u16::<LittleEndian>(frame_header.number_of_materials).unwrap();
        result.resize(0x44, 0);

        result
    }

    async fn read_image_raw(&mut self, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_headers = read_and_parse!(
            self.file,
            base_offset + FileHeader::SIZE as u64,
            file_header.frame_count,
            ImageFrameHeader
        )
        .await?;
        let sizes_table_base = base_offset + FileHeader::SIZE as u64 + file_header.frame_count as u64 * ImageFrameHeader::SIZE as u64;

        let block_count = frame_headers.iter().map(|x| x.block_count as usize).sum();
        let mut block_offsets = Vec::with_capacity(block_count);

        for frame_header in &frame_headers {
            let block_sizes = self
                .read_block_sizes(
                    sizes_table_base + frame_header.sizes_table_offset as u64,
                    frame_header.block_count as usize,
                )
                .await?;
            block_offsets.extend(Self::block_sizes_to_offset(
                &block_sizes,
                base_offset + file_header.header_length as u64 + frame_header.block_offset as u64,
            ));
        }

        let additional_header = self
            .file
            .read_to_vec(base_offset + file_header.header_length as u64, frame_headers[0].block_offset as usize)
            .await?;

        Ok(SqPackRawFile {
            additional_header,
            blocks: self.read_blocks(block_offsets.into_iter()).await?,
        })
    }
}
