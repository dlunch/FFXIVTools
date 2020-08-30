use alloc::{boxed::Box, vec::Vec};

use async_trait::async_trait;
use futures::{
    future::FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use hashbrown::HashMap;

use sqpack::{Package, Result, SqPackFileReference, SqPackReader};

#[async_trait]
pub trait BatchablePackage: Sync + Send {
    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<HashMap<SqPackFileReference, Vec<u8>>>;
}

#[async_trait]
impl BatchablePackage for SqPackReader {
    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<HashMap<SqPackFileReference, Vec<u8>>> {
        references
            .iter()
            .map(|reference| self.read_file_by_reference(reference).map(move |x| Ok(((*reference).to_owned(), x?))))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<HashMap<_, _>>()
            .await
    }
}
