use std::io;

use async_trait::async_trait;
use log::debug;

use super::FileProvider;
use crate::reference::SqPackFileHash;

pub struct FileProviderWeb {
    base_uri: String,
}

impl FileProviderWeb {
    pub fn new(base_uri: &str) -> Self {
        Self {
            base_uri: base_uri.to_owned(),
        }
    }

    async fn fetch(&self, hash: &SqPackFileHash) -> reqwest::Result<Vec<u8>> {
        let uri = format!("{}{}/{}/{}", self.base_uri, hash.folder, hash.file, hash.path,);

        debug!("Fetching {}", uri);

        let result = reqwest::get(&uri).await?.bytes().await?;
        Ok(Vec::from(&result[..]))
    }
}

#[async_trait]
impl FileProvider for FileProviderWeb {
    async fn read_file(&self, hash: &SqPackFileHash) -> io::Result<Vec<u8>> {
        self.fetch(hash).await.map_err(|x| {
            debug!("Error downloading file");

            io::Error::new(io::ErrorKind::NotFound, x.to_string())
        })
    }
}
