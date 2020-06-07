use async_trait::async_trait;
use futures::io::AsyncReadExt;
use log::debug;

use util::cast;

use super::ExtractedFileProvider;
use crate::error::{Result, SqPackReaderError};
use crate::reference::{SqPackFileHash, SqPackFileReference};

#[repr(C)]
struct BulkItemHeader {
    folder_hash: u32,
    file_hash: u32,
    path_hash: u32,
    compressed_size: u32,
}

pub struct ExtractedFileProviderWeb {
    base_uri: String,
    progress_callback: Option<Box<dyn Fn(usize, usize) + Sync + Send + 'static>>,
}

impl ExtractedFileProviderWeb {
    pub fn new(base_uri: &str) -> Self {
        Self {
            base_uri: base_uri.to_owned(),
            progress_callback: None,
        }
    }

    pub fn with_progress<F>(base_uri: &str, progress_callback: F) -> Self
    where
        F: Fn(usize, usize) + Sync + Send + 'static,
    {
        Self {
            base_uri: base_uri.to_owned(),
            progress_callback: Some(Box::new(progress_callback)),
        }
    }

    async fn download(&self, uri: &str) -> Result<Vec<u8>> {
        debug!("Fetching {}", uri);

        let mut response = surf::get(uri).await.map_err(|x| {
            debug!("Error downloading file, {}", x);

            SqPackReaderError::NoSuchFile
        })?;
        let length_header = response.header("content-length").unwrap();
        let total_length = length_header.as_str().parse::<usize>().unwrap();

        let mut result = vec![0; total_length];
        let mut cursor = 0;
        loop {
            cursor += response.read(&mut result[cursor..]).await?;
            if let Some(progress_callback) = &self.progress_callback {
                progress_callback(cursor, total_length)
            }

            if cursor == total_length {
                break;
            }
        }

        Ok(result)
    }

    async fn fetch(&self, hash: &SqPackFileHash) -> Result<Vec<u8>> {
        let uri = format!("{}{}/{}/{}", self.base_uri, hash.folder, hash.file, hash.path);

        self.download(&uri).await
    }

    async fn fetch_many(&self, references: &[&SqPackFileReference]) -> Result<Vec<(SqPackFileHash, Vec<u8>)>> {
        let uri = format!(
            "{}bulk/{}",
            self.base_uri,
            references
                .iter()
                .map(|x| format!("{:x}-{:x}-{:x}", x.hash.folder, x.hash.file, x.hash.path))
                .collect::<Vec<_>>()
                .join(".")
        );

        let data = self.download(&uri).await?;

        Ok((0..references.len())
            .scan(0, |cursor, _| {
                let header = cast::<BulkItemHeader>(&data[*cursor..]);
                let data_begin = *cursor + core::mem::size_of::<BulkItemHeader>();
                let data_end = data_begin + header.compressed_size as usize;

                *cursor = data_end;
                Some((
                    SqPackFileHash::from_raw_hash(header.path_hash, header.folder_hash, header.file_hash),
                    Vec::from(&data[data_begin..data_end]),
                ))
            })
            .collect::<Vec<_>>())
    }
}

#[async_trait]
impl ExtractedFileProvider for ExtractedFileProviderWeb {
    async fn read_file(&self, hash: &SqPackFileHash) -> Result<Vec<u8>> {
        self.fetch(hash).await
    }

    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<Vec<(SqPackFileHash, Vec<u8>)>> {
        self.fetch_many(references).await.map_err(|x| {
            debug!("Error downloading file, {}", x);

            SqPackReaderError::NoSuchFile
        })
    }

    async fn read_file_size(&self, _: &SqPackFileHash) -> Option<u64> {
        None
    }
}
