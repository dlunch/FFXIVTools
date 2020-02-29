mod file_provider_file;
mod file_provider_web;

pub use file_provider_file::FileProviderFile;
pub use file_provider_web::FileProviderWeb;

use std::io;

use async_trait::async_trait;
use bytes::Bytes;

use crate::common::SqPackFileReference;

#[async_trait]
pub trait FileProvider: Sync + Send {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Bytes>;
}
