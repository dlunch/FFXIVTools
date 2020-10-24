use crate::Region;

use alloc::{boxed::Box, format, sync::Arc, vec::Vec};
use core::{marker::PhantomData, time::Duration};

use async_trait::async_trait;
use wasm_bindgen_futures::spawn_local;
use wasm_timer::Delay;

use sqpack::{Package, Result, SqPackFileReference};
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct WasmPackage<'a> {
    package: Arc<BatchedPackage<'a>>,
    phantom: PhantomData<&'a u8>,
}

impl<'a> WasmPackage<'a> {
    pub fn new(region: &Region) -> Self {
        let uri = format!("{}_{}", region.name, region.version);
        let provider = ExtractedFileProviderWeb::new(&format!("https://ffxiv-data.dlunch.net/compressed/{}/", uri));

        let result = Arc::new(BatchedPackage::new(SqPackReaderExtractedFile::new(provider)));
        let package = result.clone();

        spawn_local(async move {
            loop {
                if Arc::strong_count(&package) == 1 {
                    break;
                }

                package.poll().await.unwrap();
                Delay::new(Duration::from_millis(16)).await.unwrap();
            }
        });

        Self {
            package: result,
            phantom: PhantomData,
        }
    }
}

#[async_trait]
impl Package for WasmPackage<'_> {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>> {
        self.package.read_file_by_reference(reference).await
    }
}
