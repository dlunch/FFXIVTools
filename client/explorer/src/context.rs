use std::rc::Rc;

use common::{regions, WasmPackage};

use crate::file_list::FileList;

static mut INSTANCE: Option<Context> = None;

pub struct Context {
    pub package: Rc<WasmPackage>,
    pub file_list: FileList,
}

impl Context {
    pub async fn get() -> &'static Self {
        // not threadsafe, but wasm is threadless environment
        unsafe {
            match &INSTANCE {
                Some(x) => x,
                None => {
                    let instance = Context::new().await;
                    INSTANCE = Some(instance);

                    INSTANCE.as_ref().unwrap()
                }
            }
        }
    }

    async fn new() -> Self {
        let package = Rc::new(WasmPackage::new(&regions()[0]).await);
        let file_list = FileList::new(package.clone());

        Self { package, file_list }
    }
}
