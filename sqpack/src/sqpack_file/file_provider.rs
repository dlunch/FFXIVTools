use std::io;

use async_trait::async_trait;

use crate::common::SqPackFileReference;

#[async_trait]
pub trait FileProvider {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>>;
}
