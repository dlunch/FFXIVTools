use std::io;
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};
use nom::number::complete::le_u16;
use nom::{count, named_args};
use tokio::fs::File;
use tokio::sync::Mutex;

use super::definition::{DefaultFrameHeader, FileHeader, ImageFrameHeader, ModelFrameHeader, FILE_TYPE_DEFAULT, FILE_TYPE_IMAGE, FILE_TYPE_MODEL};
use crate::common::{decode_block, ReadExt};

pub struct SqPackData {
    file: Mutex<File>,
}

impl SqPackData {
    pub async fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path).await?;

        Ok(Self { file: Mutex::new(file) })
    }

    pub async fn read(&self, offset: u64) -> io::Result<Vec<u8>> {
        let mut file = self.file.lock().await;

        let file_header = read_and_parse!(file, offset, FileHeader).await?;
        match file_header.file_type {
            FILE_TYPE_DEFAULT => Ok(Self::read_default(&mut file, offset, file_header).await?),
            FILE_TYPE_MODEL => Ok(Self::read_model(&mut file, offset, file_header).await?),
            FILE_TYPE_IMAGE => Ok(Self::read_image(&mut file, offset, file_header).await?),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect header")),
        }
    }

    async fn read_default(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);
        let frame_headers = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, DefaultFrameHeader).await?;

        for frame_header in frame_headers {
            let offset = base_offset + file_header.header_length as u64 + frame_header.block_offset as u64;
            let block = file.read_to_vec(offset, frame_header.block_size as usize).await?;

            let decoded = decode_block(&block);
            result.extend(decoded.data());
        }

        Ok(result)
    }

    async fn read_block_sizes(file: &mut File, offset: u64, count: usize) -> io::Result<Vec<u16>> {
        let block_size_data = file.read_to_vec(offset, count * std::mem::size_of::<u16>()).await?;

        named_args!(parse_block_sizes(count: usize)<Vec<u16>>, count!(le_u16, count));
        let (_, block_sizes) = parse_block_sizes(&block_size_data, count).unwrap();

        Ok(block_sizes)
    }

    async fn read_contiguous_blocks(file: &mut File, base_offset: u64, block_sizes: &[u16]) -> io::Result<Vec<u8>> {
        let total_size = block_sizes.iter().map(|x| *x as usize).sum();

        Ok(file.read_to_vec(base_offset, total_size).await?)
    }

    fn decode_contiguous_blocks_into(blocks: Vec<u8>, block_sizes: &[u16], result: &mut Vec<u8>) {
        let mut offset = 0usize;

        for &block_size in block_sizes {
            let decoded = decode_block(&blocks[offset..offset + block_size as usize]);
            result.extend(decoded.data());

            offset += block_size as usize;
        }
    }

    async fn read_model(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);
        let frame_header = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, ModelFrameHeader).await?;

        result.extend(Self::serialize_model_header(&frame_header));

        let total_block_count = frame_header.block_counts.iter().sum::<u16>() as usize;
        let sizes_offset = base_offset + FileHeader::SIZE as u64 + ModelFrameHeader::SIZE as u64;
        let block_sizes = Self::read_block_sizes(file, sizes_offset, total_block_count).await?;

        let block_base_offset = base_offset + file_header.header_length as u64 + frame_header.offsets[0] as u64;
        let blocks = Self::read_contiguous_blocks(file, block_base_offset, &block_sizes).await?;
        Self::decode_contiguous_blocks_into(blocks, &block_sizes, &mut result);

        Ok(result)
    }

    fn serialize_model_header(frame_header: &ModelFrameHeader) -> Vec<u8> {
        pub const MODEL_HEADER_SIZE: usize = 4;
        let mut result = Vec::with_capacity(MODEL_HEADER_SIZE);

        result.write_u16::<LittleEndian>(frame_header.number_of_meshes).unwrap();
        result.write_u16::<LittleEndian>(frame_header.number_of_materials).unwrap();

        result
    }

    async fn read_image(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);
        let frame_headers = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, ImageFrameHeader).await?;
        let sizes_table_base = base_offset + FileHeader::SIZE as u64 + file_header.frame_count as u64 * ImageFrameHeader::SIZE as u64;

        let additional_header = file
            .read_to_vec(base_offset + file_header.header_length as u64, frame_headers[0].block_offset as usize)
            .await?;
        result.extend(additional_header);

        for frame_header in frame_headers {
            let block_sizes = Self::read_block_sizes(
                file,
                sizes_table_base + frame_header.sizes_table_offset as u64,
                frame_header.block_count as usize,
            )
            .await?;

            let block_base_offset = base_offset + file_header.header_length as u64 + frame_header.block_offset as u64;
            let blocks = Self::read_contiguous_blocks(file, block_base_offset, &block_sizes).await?;
            Self::decode_contiguous_blocks_into(blocks, &block_sizes, &mut result);
        }

        Ok(result)
    }
}
