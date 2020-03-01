use std::io;

use async_trait::async_trait;
use bytes::Bytes;

use crate::common::SqPackFileReference;

#[async_trait]
pub trait Package: Sync + Send {
    async fn read_file(&self, path: &str) -> io::Result<Bytes> {
        let reference = SqPackFileReference::new(path);

        self.read_file_by_reference(&reference).await
    }

    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes>;
}
