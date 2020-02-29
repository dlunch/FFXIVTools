use std::io;

use async_trait::async_trait;

use crate::common::{decode_compressed_data, SqPackFileReference};
use crate::file_provider::FileProvider;
use crate::package::Package;

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
}

#[async_trait]
impl Package for SqPackReaderFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let data = self.provider.read_file(reference).await?;

        Ok(decode_compressed_data(&data))
    }
}
