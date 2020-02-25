use std::io;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use byteorder::{LittleEndian, ReadBytesExt};
use tokio::fs::File;

use crate::common::{ReadExt, SqPackFileReference, SqPackRawFile, MODEL_HEADER_SIZE};
use crate::package::Package;

pub struct SqPackFile {
    base_dir: PathBuf,
}

impl SqPackFile {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        Ok(SqPackFile {
            base_dir: base_dir.to_owned(),
        })
    }

    fn find_path(&self, reference: &SqPackFileReference) -> io::Result<PathBuf> {
        let mut path = self.base_dir.clone();

        path.push(reference.folder_hash.to_string());
        path.push(reference.file_hash.to_string());

        if path.exists() {
            Ok(path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
        }
    }

    async fn decode_file(path: &Path) -> io::Result<Vec<u8>> {
        let mut file = File::open(path).await?;

        const FILE_HEADER_SIZE: usize = 12;

        let file_header = file.read_to_vec(0, FILE_HEADER_SIZE).await?;
        let mut reader = Cursor::new(file_header);

        let _uncompressed_size = reader.read_u32::<LittleEndian>()?;
        let additional_header_size = reader.read_u32::<LittleEndian>()?;
        let block_count = reader.read_u32::<LittleEndian>()?;

        let mut additional_header = file.read_to_vec(FILE_HEADER_SIZE as u64, additional_header_size as usize).await?;
        if additional_header_size == 4 {
            additional_header.extend(vec![0; MODEL_HEADER_SIZE])
        }

        let blocks = crate::common::SqPackDataBlock::with_compressed_data(
            &mut file,
            FILE_HEADER_SIZE as u64 + additional_header_size as u64,
            block_count as usize,
        )
        .await?;

        let raw_file = SqPackRawFile::new(additional_header, blocks);

        Ok(raw_file.decode())
    }
}

#[async_trait]
impl Package for SqPackFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let path = self.find_path(reference)?;

        Ok(Self::decode_file(&path).await?)
    }
}
