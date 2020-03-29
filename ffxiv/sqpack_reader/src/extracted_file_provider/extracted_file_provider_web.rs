use std::io;

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;

use super::ExtractedFileProvider;
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

    async fn fetch(&self, hash: &SqPackFileHash) -> reqwest::Result<Bytes> {
        let uri = format!("{}{}/{}/{}", self.base_uri, hash.folder, hash.file, hash.path,);

        debug!("Fetching {}", uri);

        Ok(reqwest::get(&uri).await?.bytes().await?)
    }
}

#[async_trait]
impl ExtractedFileProvider for ExtractedFileProviderWeb {
    async fn read_file(&self, hash: &SqPackFileHash) -> io::Result<Bytes> {
        self.fetch(hash).await.map_err(|x| {
            debug!("Error downloading file");

            io::Error::new(io::ErrorKind::NotFound, x.to_string())
        })
    }

    async fn read_file_size(&self, _: &SqPackFileHash) -> Option<u64> {
        None
    }
}
