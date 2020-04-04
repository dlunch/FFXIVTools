cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod extracted_file_provider_local;
        mod extracted_file_provider_web;

        pub use extracted_file_provider_local::ExtractedFileProviderLocal;
        pub use extracted_file_provider_web::ExtractedFileProviderWeb;
    }
}

use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Result;
use crate::reference::SqPackFileHash;

#[async_trait]
pub trait ExtractedFileProvider: Sync + Send {
    async fn read_file(&self, hash: &SqPackFileHash) -> Result<Bytes>;
    async fn read_file_size(&self, hash: &SqPackFileHash) -> Option<u64>;
}
