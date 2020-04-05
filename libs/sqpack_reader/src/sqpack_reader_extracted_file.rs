use alloc::boxed::Box;

use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Result;
use crate::extracted_file_provider::ExtractedFileProvider;
use crate::package::Package;
use crate::raw_file::SqPackRawFile;
use crate::reference::{SqPackFileHash, SqPackFileReference};

pub struct SqPackReaderExtractedFile {
    provider: Box<dyn ExtractedFileProvider>,
}

impl SqPackReaderExtractedFile {
    pub fn new<T>(provider: T) -> Result<Self>
    where
        T: ExtractedFileProvider + 'static,
    {
        Ok(Self {
            provider: Box::new(provider),
        })
    }

    pub async fn read_as_compressed_by_hash(&self, hash: &SqPackFileHash) -> Result<Bytes> {
        self.provider.read_file(hash).await
    }

    pub async fn read_compressed_size_by_hash(&self, hash: &SqPackFileHash) -> Option<u64> {
        self.provider.read_file_size(hash).await
    }
}

#[async_trait]
impl Package for SqPackReaderExtractedFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Bytes> {
        let data = self.read_as_compressed_by_hash(&reference.hash).await?;

        Ok(SqPackRawFile::from_compressed_file(data).into_decoded())
    }

    async fn read_as_compressed_by_reference(&self, reference: &SqPackFileReference) -> Result<Bytes> {
        self.read_as_compressed_by_hash(&reference.hash).await
    }
}
