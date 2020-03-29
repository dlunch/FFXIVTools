use std::io;

use async_trait::async_trait;
use bytes::Bytes;

use crate::file_provider::FileProvider;
use crate::package::Package;
use crate::raw_file::SqPackRawFile;
use crate::reference::{SqPackFileHash, SqPackFileReference};

pub struct SqPackReaderFile {
    provider: Box<dyn FileProvider>,
}

impl SqPackReaderFile {
    pub fn new<T>(provider: T) -> io::Result<Self>
    where
        T: FileProvider + 'static,
    {
        Ok(Self {
            provider: Box::new(provider),
        })
    }

    pub async fn read_as_compressed_by_hash(&self, hash: &SqPackFileHash) -> io::Result<Bytes> {
        self.provider.read_file(hash).await
    }

    pub async fn read_compressed_size_by_hash(&self, hash: &SqPackFileHash) -> Option<u64> {
        self.provider.read_file_size(hash).await
    }
}

#[async_trait]
impl Package for SqPackReaderFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        let data = self.read_as_compressed_by_hash(&reference.hash).await?;

        Ok(SqPackRawFile::from_compressed_file(data).into_decoded())
    }

    async fn read_as_compressed_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        self.read_as_compressed_by_hash(&reference.hash).await
    }
}
