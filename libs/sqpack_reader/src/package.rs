use alloc::boxed::Box;
use std::collections::HashMap;

use async_trait::async_trait;
use log::debug;

use crate::error::Result;
use crate::reference::SqPackFileReference;

#[async_trait]
pub trait Package: Sync + Send {
    async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        debug!("Reading {}", path);

        let reference = SqPackFileReference::new(path);
        let result = self.read_file_by_reference(&reference).await;

        if result.is_err() {
            debug!("No such file {}", path);
        }
        result
    }

    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait BatchablePackage: Sync + Send {
    async fn read_many(&self, references: &[&SqPackFileReference]) -> Result<HashMap<SqPackFileReference, Vec<u8>>>;
}
