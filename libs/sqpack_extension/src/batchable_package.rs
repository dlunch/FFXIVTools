use alloc::{boxed::Box, vec::Vec};

use async_trait::async_trait;
use hashbrown::HashMap;

use sqpack::{Result, SqPackFileReference};

#[async_trait]
pub trait BatchablePackage: Sync + Send {
    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<HashMap<SqPackFileReference, Vec<u8>>>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use futures::{
            future::FutureExt,
            stream::{FuturesUnordered, TryStreamExt},
        };
        use sqpack::{Package, SqPackReader};

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
    }
}
