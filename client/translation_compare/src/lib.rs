extern crate alloc;

mod app;
mod list;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn start(base_url: &str) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let props = app::Props {
        base_url: base_url.to_owned(),
    };
    yew::start_app_with_props::<app::App>(props);
}

#[wasm_bindgen(start)]
pub fn main() {}
