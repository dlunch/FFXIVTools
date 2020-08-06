use alloc::{boxed::Box, vec::Vec};

mod extracted_file_provider_web;
pub use extracted_file_provider_web::ExtractedFileProviderWeb;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod extracted_file_provider_local;
        pub use extracted_file_provider_local::ExtractedFileProviderLocal;
    }
}

use async_trait::async_trait;

use crate::error::Result;
use crate::reference::{SqPackFileHash, SqPackFileReference};

#[async_trait]
pub trait ExtractedFileProvider: Sync + Send {
    async fn read_file(&self, hash: &SqPackFileHash) -> Result<Vec<u8>>;
    async fn read_file_size(&self, hash: &SqPackFileHash) -> Option<u64>;
    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<Vec<(SqPackFileHash, Vec<u8>)>>;
}
