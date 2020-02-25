use std::io;
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};
use nom::number::complete::le_u16;
use nom::{count, named_args};
use tokio::fs::File;
use tokio::sync::Mutex;

use super::definition::{DefaultFrameHeader, FileHeader, ImageFrameHeader, ModelFrameHeader, FILE_TYPE_DEFAULT, FILE_TYPE_IMAGE, FILE_TYPE_MODEL};
use crate::common::ReadExt;
use crate::common::SqPackDataBlock;
use crate::common::SqPackRawFile;

pub struct SqPackData {
    file: Mutex<File>,
}

impl SqPackData {
    pub async fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path).await?;

        Ok(Self { file: Mutex::new(file) })
    }

    pub async fn read(&self, offset: u64) -> io::Result<Vec<u8>> {
        let raw_file = self.read_raw_file(offset).await?;

        Ok(raw_file.decode())
    }

    async fn read_raw_file(&self, offset: u64) -> io::Result<SqPackRawFile> {
        let mut file = self.file.lock().await;
        let file_header = read_and_parse!(file, offset, FileHeader).await?;

        Ok(Self::read_raw(&mut file, offset, file_header).await?)
    }

    async fn read_raw(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        match file_header.file_type {
            FILE_TYPE_DEFAULT => Ok(Self::read_default_raw(file, base_offset, file_header).await?),
            FILE_TYPE_MODEL => Ok(Self::read_model_raw(file, base_offset, file_header).await?),
            FILE_TYPE_IMAGE => Ok(Self::read_image_raw(file, base_offset, file_header).await?),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect header")),
        }
    }

    async fn read_blocks(file: &mut File, block_offsets: impl Iterator<Item = u64>) -> io::Result<Vec<SqPackDataBlock>> {
        let mut result = Vec::with_capacity(block_offsets.size_hint().0);

        for block_offset in block_offsets {
            result.push(SqPackDataBlock::new(file, block_offset).await?);
        }

        Ok(result)
    }

    async fn read_default_raw(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_headers = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, DefaultFrameHeader).await?;

        let block_offsets = frame_headers
            .iter()
            .map(|x| base_offset + file_header.header_length as u64 + x.block_offset as u64);

        Ok(SqPackRawFile::new(Vec::new(), Self::read_blocks(file, block_offsets).await?))
    }

    async fn read_block_sizes(file: &mut File, offset: u64, count: usize) -> io::Result<Vec<u16>> {
        let block_size_data = file.read_to_vec(offset, count * std::mem::size_of::<u16>()).await?;

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

    async fn read_model_raw(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_header = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, ModelFrameHeader).await?;

        let total_block_count = frame_header.block_counts.iter().sum::<u16>() as usize;
        let sizes_offset = base_offset + FileHeader::SIZE as u64 + ModelFrameHeader::SIZE as u64;
        let block_sizes = Self::read_block_sizes(file, sizes_offset, total_block_count).await?;
        let block_offsets = Self::block_sizes_to_offset(
            &block_sizes,
            base_offset + file_header.header_length as u64 + frame_header.offsets[0] as u64,
        );

        Ok(SqPackRawFile::new(
            Self::serialize_model_header(&frame_header),
            Self::read_blocks(file, block_offsets).await?,
        ))
    }

    fn serialize_model_header(frame_header: &ModelFrameHeader) -> Vec<u8> {
        let mut result = Vec::with_capacity(0x44);

        result.write_u16::<LittleEndian>(frame_header.number_of_meshes).unwrap();
        result.write_u16::<LittleEndian>(frame_header.number_of_materials).unwrap();
        result.resize(0x44, 0);

        result
    }

    async fn read_image_raw(file: &mut File, base_offset: u64, file_header: FileHeader) -> io::Result<SqPackRawFile> {
        let frame_headers = read_and_parse!(file, base_offset + FileHeader::SIZE as u64, file_header.frame_count, ImageFrameHeader).await?;
        let sizes_table_base = base_offset + FileHeader::SIZE as u64 + file_header.frame_count as u64 * ImageFrameHeader::SIZE as u64;

        let block_count = frame_headers.iter().map(|x| x.block_count as usize).sum();
        let mut block_offsets = Vec::with_capacity(block_count);

        for frame_header in &frame_headers {
            let block_sizes = Self::read_block_sizes(
                file,
                sizes_table_base + frame_header.sizes_table_offset as u64,
                frame_header.block_count as usize,
            )
            .await?;
            block_offsets.extend(Self::block_sizes_to_offset(
                &block_sizes,
                base_offset + file_header.header_length as u64 + frame_header.block_offset as u64,
            ));
        }

        let additional_header = file
            .read_to_vec(base_offset + file_header.header_length as u64, frame_headers[0].block_offset as usize)
            .await?;

        Ok(SqPackRawFile::new(
            additional_header,
            Self::read_blocks(file, block_offsets.into_iter()).await?,
        ))
    }
}
