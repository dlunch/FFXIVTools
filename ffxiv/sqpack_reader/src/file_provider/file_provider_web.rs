use std::io;

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;

use super::FileProvider;
use crate::reference::SqPackFileReference;

pub struct FileProviderWeb {
    base_uri: String,
}

impl FileProviderWeb {
    pub fn new(base_uri: &str) -> Self {
        Self {
            base_uri: base_uri.to_owned(),
        }
    }

    async fn fetch(&self, reference: &SqPackFileReference) -> reqwest::Result<Bytes> {
        let uri = format!(
            "{}{}/{}/{}",
            self.base_uri, reference.folder_hash, reference.file_hash, reference.path_hash,
        );

        debug!("Fetching {}", uri);

        let result = reqwest::get(&uri).await?.bytes().await?;
        Ok(result)
    }
}

#[async_trait]
impl FileProvider for FileProviderWeb {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        self.fetch(reference).await.map_err(|x| {
            debug!("Error downloading file");

            io::Error::new(io::ErrorKind::NotFound, x.to_string())
        })
    }
}
