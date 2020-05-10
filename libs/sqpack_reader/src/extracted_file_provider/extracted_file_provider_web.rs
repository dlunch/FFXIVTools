use async_trait::async_trait;
use futures::stream::StreamExt;
use log::debug;

use super::ExtractedFileProvider;
use crate::error::{Result, SqPackReaderError};
use crate::reference::SqPackFileHash;

pub struct ExtractedFileProviderWeb {
    base_uri: String,
    progress_callback: Option<Box<dyn Fn(usize, usize) -> () + Sync + Send + 'static>>,
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
        F: Fn(usize, usize) -> () + Sync + Send + 'static,
    {
        Self {
            base_uri: base_uri.to_owned(),
            progress_callback: Some(Box::new(progress_callback)),
        }
    }

    async fn fetch(&self, hash: &SqPackFileHash) -> reqwest::Result<Vec<u8>> {
        let uri = format!("{}{}/{}/{}", self.base_uri, hash.folder, hash.file, hash.path);

        debug!("Fetching {}", uri);

        let response = reqwest::get(&uri).await?.error_for_status()?;
        let total_length = response.content_length().unwrap() as usize;
        let mut result = Vec::with_capacity(total_length);
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            if let Some(progress_callback) = &self.progress_callback {
                progress_callback(result.len(), total_length)
            }
            result.extend_from_slice(&item?[..]);
        }

        Ok(result)
    }
}

#[async_trait]
impl ExtractedFileProvider for ExtractedFileProviderWeb {
    async fn read_file(&self, hash: &SqPackFileHash) -> Result<Vec<u8>> {
        self.fetch(hash).await.map_err(|x| {
            debug!("Error downloading file, {}", x);

            SqPackReaderError::NoSuchFile
        })
    }

    async fn read_file_size(&self, _: &SqPackFileHash) -> Option<u64> {
        None
    }
}
