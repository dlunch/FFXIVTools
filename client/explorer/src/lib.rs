mod app;
mod components;
mod context;
mod file_list;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start(base_url: String) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    spawn_local(async move {
        context::Context::init(&base_url).await;
    });

    yew::start_app::<app::App>();
}
