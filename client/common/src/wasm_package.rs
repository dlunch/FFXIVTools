use crate::Region;

use alloc::{boxed::Box, format, sync::Arc, vec::Vec};

use async_trait::async_trait;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

use sqpack::{Package, Result, SqPackFileReference};
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};
pub struct WasmPackage {
    package: Arc<BatchedPackage>,
}

impl WasmPackage {
    pub async fn new(region: &Region, base_url: &str) -> Self {
        let uri = format!("{}_{}", region.name, region.version);
        let provider = ExtractedFileProviderWeb::new(&format!("{base_url}/{uri}/"));

        let result = Arc::new(BatchedPackage::new(SqPackReaderExtractedFile::new(provider)));
        let package = result.clone();

        spawn_local(async move {
            loop {
                if Arc::strong_count(&package) == 1 {
                    break;
                }

                package.poll().await.unwrap();
                TimeoutFuture::new(16).await
            }
        });

        Self { package: result }
    }
}

#[async_trait]
impl Package for WasmPackage {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Vec<u8>> {
        self.package.read_file_by_reference(reference).await
    }
}
