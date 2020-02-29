use std::io;

use async_trait::async_trait;
use bytes::Bytes;

use super::file_provider::FileProvider;
use crate::common::SqPackFileReference;

pub struct FileProviderWeb {
    base_uri: String,
}

impl FileProviderWeb {
    pub fn new(base_uri: &str) -> Box<Self> {
        Box::new(Self {
            base_uri: base_uri.to_owned(),
        })
    }

    async fn do_read(&self, reference: &SqPackFileReference) -> reqwest::Result<Bytes> {
        let uri = format!(
            "{}{}/{}/{}",
            self.base_uri, reference.folder_hash, reference.file_hash, reference.path_hash,
        );

        Ok(reqwest::get(&uri).await?.bytes().await?)
    }
}

#[async_trait]
impl FileProvider for FileProviderWeb {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        self.do_read(reference)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::NotFound, x.to_string()))
    }
}
