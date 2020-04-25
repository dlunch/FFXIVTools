use async_trait::async_trait;
use log::debug;

use super::ExtractedFileProvider;
use crate::error::{Result, SqPackReaderError};
use crate::reference::SqPackFileHash;

pub struct ExtractedFileProviderWeb {
    base_uri: String,
}

impl ExtractedFileProviderWeb {
    pub fn new(base_uri: &str) -> Self {
        Self {
            base_uri: base_uri.to_owned(),
        }
    }

    async fn fetch(&self, hash: &SqPackFileHash) -> reqwest::Result<Vec<u8>> {
        let uri = format!("{}{}/{}/{}", self.base_uri, hash.folder, hash.file, hash.path);

        debug!("Fetching {}", uri);

        let result = reqwest::get(&uri).await?.error_for_status()?;
        Ok(Vec::from(&result.bytes().await?[..]))
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
