use alloc::boxed::Box;
use std::collections::HashMap;

use async_trait::async_trait;
use futures::future;
use futures::FutureExt;

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
        future::join_all(
            references
                .iter()
                .map(|reference| self.read_file_by_reference(reference).map(move |x| Ok(((*reference).to_owned(), x?)))),
        )
        .await
        .into_iter()
        .collect::<Result<HashMap<_, _>>>()
    }
}
