use std::io;

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;

use crate::reference::SqPackFileReference;

#[async_trait]
pub trait Package: Sync + Send {
    async fn read_file(&self, path: &str) -> io::Result<Bytes> {
        debug!("Reading {}", path);

        let reference = SqPackFileReference::new(path);
        let result = self.read_file_by_reference(&reference).await;

        if result.is_err() {
            debug!("No such file {}", path);
        }
        result
    }

    async fn read_as_compressed(&self, path: &str) -> io::Result<Bytes> {
        debug!("Reading {}", path);

        let reference = SqPackFileReference::new(path);
        let result = self.read_as_compressed_by_reference(&reference).await;

        if result.is_err() {
            debug!("No such file {}", path);
        }
        result
    }

    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes>;
    async fn read_as_compressed_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes>;
}
