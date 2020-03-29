mod file_provider_file;
mod file_provider_web;

pub use file_provider_file::FileProviderFile;
pub use file_provider_web::FileProviderWeb;

use std::io;

use async_trait::async_trait;
use bytes::Bytes;

use crate::reference::SqPackFileHash;

#[async_trait]
pub trait FileProvider: Sync + Send {
    async fn read_file(&self, hash: &SqPackFileHash) -> io::Result<Bytes>;
    async fn read_file_size(&self, hash: &SqPackFileHash) -> Option<u64>;
}
