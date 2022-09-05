use std::rc::Rc;

use common::{regions, WasmPackage};

use crate::file_list::FileList;

static mut INSTANCE: Option<AppContext> = None;

pub struct AppContext {
    pub package: Rc<WasmPackage>,
    pub file_list: FileList,
}

impl AppContext {
    pub fn get() -> &'static Self {
        return unsafe { INSTANCE.as_ref() }.unwrap();
    }

    pub async fn init(base_url: &str) {
        let package = Rc::new(WasmPackage::new(&regions()[0], base_url).await);
        let file_list = FileList::new(package.clone());

        let instance = Self { package, file_list };

        unsafe {
            INSTANCE = Some(instance);
        }
    }
}
