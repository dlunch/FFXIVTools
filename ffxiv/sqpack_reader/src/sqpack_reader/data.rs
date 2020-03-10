use std::io;
use std::path::Path;

use bytes::{Buf, BufMut, Bytes};
use tokio::fs::File;
use tokio::sync::Mutex;

use super::definition::{DefaultFrameInfo, FileHeader, ImageFrameInfo, ModelFrameInfo, FILE_TYPE_DEFAULT, FILE_TYPE_IMAGE, FILE_TYPE_MODEL};
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
        let frame_headers = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, DefaultFrameInfo).await?;

        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);
        for frame_header in frame_headers {
            let offset = base_offset + file_header.header_length as u64 + frame_header.block_offset as u64;
            let block = file.read_bytes(offset, frame_header.block_size as usize).await?;

            let (_, decoded) = decode_block(block);
            result.extend(decoded);
        }

        Ok(result)
    }

    async fn read_block_sizes(file: &mut File, offset: u64, count: usize) -> io::Result<Vec<u16>> {
        let mut block_size_data = file.read_bytes(offset, count * std::mem::size_of::<u16>()).await?;

        Ok((0..count).map(move |_| block_size_data.get_u16_le()).collect())
    }

    async fn read_contiguous_blocks(file: &mut File, base_offset: u64, block_sizes: &[u16]) -> io::Result<Bytes> {
        let total_size = block_sizes.iter().map(|x| *x as usize).sum();

        Ok(file.read_bytes(base_offset, total_size).await?)
    }

    fn decode_contiguous_blocks(block_data: Bytes, block_sizes: &[u16]) -> Vec<Bytes> {
        let mut result = Vec::with_capacity(block_sizes.len());
        let mut offset = 0usize;

        for &block_size in block_sizes {
            let (_, decoded) = decode_block(block_data.slice(offset..offset + block_size as usize));
            result.push(decoded);

            offset += block_size as usize;
        }

        result
    }

    async fn read_model(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<Vec<u8>> {
        let frame_info = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, ModelFrameInfo).await?;
        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);

        // header
        result.put_u16_le(frame_info.number_of_meshes);
        result.put_u16_le(frame_info.number_of_materials);

        let total_block_count = frame_info.block_counts.iter().sum::<u16>() as usize;
        let sizes_offset = base_offset + FileHeader::SIZE as u64 + ModelFrameInfo::SIZE as u64;
        let block_sizes = Self::read_block_sizes(file, sizes_offset, total_block_count).await?;

        let block_base_offset = base_offset + file_header.header_length as u64 + frame_info.offsets[0] as u64;
        let block_data = Self::read_contiguous_blocks(file, block_base_offset, &block_sizes).await?;
        let blocks = Self::decode_contiguous_blocks(block_data, &block_sizes);

        for block in blocks {
            result.extend(block)
        }

        Ok(result)
    }

    async fn read_image(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<Vec<u8>> {
        let frame_infos = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, ImageFrameInfo).await?;
        let sizes_table_base = base_offset + FileHeader::SIZE as u64 + file_header.frame_count as u64 * ImageFrameInfo::SIZE as u64;

        let mut result = Vec::with_capacity(file_header.uncompressed_size as usize);

        let additional_header = file
            .read_bytes(base_offset + file_header.header_length as u64, frame_infos[0].block_offset as usize)
            .await?;
        result.extend(additional_header);

        for frame_info in frame_infos {
            let block_sizes = Self::read_block_sizes(
                file,
                sizes_table_base + frame_info.sizes_table_offset as u64,
                frame_info.block_count as usize,
            )
            .await?;

            let block_base_offset = base_offset + file_header.header_length as u64 + frame_info.block_offset as u64;
            let block_data = Self::read_contiguous_blocks(file, block_base_offset, &block_sizes).await?;
            let blocks = Self::decode_contiguous_blocks(block_data, &block_sizes);

            for block in blocks {
                result.extend(block);
            }
        }

        Ok(result)
    }
}
