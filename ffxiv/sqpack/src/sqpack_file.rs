use std::io;

use async_trait::async_trait;

use crate::common::{decode_compressed_data, SqPackFileReference};
use crate::file_provider::FileProvider;
use crate::package::Package;

pub struct SqPackFile {
    provider: Box<dyn FileProvider>,
}

impl SqPackFile {
    pub fn new<T>(provider: T) -> io::Result<Self>
    where
        T: FileProvider + 'static,
    {
        Ok(SqPackFile {
            provider: Box::new(provider),
        })
    }
}

#[async_trait]
impl Package for SqPackFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let data = self.provider.read_file(reference).await?;

        Ok(decode_compressed_data(&data))
    }
}
