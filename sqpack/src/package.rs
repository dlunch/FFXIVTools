use async_trait::async_trait;
use std::io;

use crate::common::SqPackFileReference;

#[async_trait]
pub trait Package {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        let reference = SqPackFileReference::new(path);

        self.read_file_by_reference(&reference).await
    }

    async fn read_file_by_reference(&self, refenrece: &SqPackFileReference) -> io::Result<Vec<u8>>;
}
