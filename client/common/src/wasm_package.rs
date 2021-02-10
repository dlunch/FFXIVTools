use crate::Region;

use alloc::{boxed::Box, format, sync::Arc, vec::Vec};
use core::time::Duration;

use async_timer::Interval;
use async_trait::async_trait;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;

use sqpack::{Package, Result, SqPackFileReference};
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[rustfmt::skip]
#[wasm_bindgen(module = "common")]
extern "C" {
    #[wasm_bindgen(js_name = "getBaseUri")]
    async fn get_base_uri() -> JsValue;
}

pub struct WasmPackage {
    package: Arc<BatchedPackage>,
}

impl WasmPackage {
    pub async fn new(region: &Region) -> Self {
        let base_uri = unsafe { get_base_uri().await.as_string().unwrap() };
        let uri = format!("{}_{}", region.name, region.version);
        let provider = ExtractedFileProviderWeb::new(&format!("{}/{}/", base_uri, uri));

        let result = Arc::new(BatchedPackage::new(SqPackReaderExtractedFile::new(provider)));
        let package = result.clone();

        spawn_local(async move {
            let mut interval = Interval::platform_new(Duration::from_millis(16));
            loop {
                if Arc::strong_count(&package) == 1 {
                    break;
                }

                package.poll().await.unwrap();
                interval.as_mut().await;
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
