use alloc::boxed::Box;
use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::Result;
use crate::extracted_file_provider::ExtractedFileProvider;
use crate::package::{BatchablePackage, Package};
use crate::raw_file::SqPackRawFile;
use crate::reference::{SqPackFileHash, SqPackFileReference};

pub struct SqPackReaderExtractedFile {
    provider: Box<dyn ExtractedFileProvider>,
}

impl SqPackReaderExtractedFile {
    pub fn new<T>(provider: T) -> Self
    where
        T: ExtractedFileProvider + 'static,
    {
        Self {
            provider: Box::new(provider),
        }
    }

    pub async fn read_as_compressed_by_hash(&self, hash: &SqPackFileHash) -> Result<Vec<u8>> {
        self.provider.read_file(hash).await
    }

    pub async fn read_compressed_size_by_hash(&self, hash: &SqPackFileHash) -> Option<u64> {
        self.provider.read_file_size(hash).await
    }
}

#[async_trait]
impl Package for SqPackReaderExtractedFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>> {
        let data = self.read_as_compressed_by_hash(&reference.hash).await?;

        Ok(SqPackRawFile::from_compressed_file(data).into_decoded())
    }
}

#[async_trait]
impl BatchablePackage for SqPackReaderExtractedFile {
    async fn read_many(&self, references: &[&SqPackFileReference]) -> Result<HashMap<SqPackFileReference, Vec<u8>>> {
        let hash_references = references.iter().map(|&x| (x.hash, x)).collect::<HashMap<_, _>>();
        let hashes = hash_references.keys().into_iter().collect::<Vec<_>>();

        Ok(self
            .provider
            .read_files(hashes.as_slice())
            .await?
            .into_iter()
            .map(|(hash, result)| ((*hash_references.get(&hash).unwrap()).clone(), result))
            .collect::<HashMap<_, _>>())
    }
}
