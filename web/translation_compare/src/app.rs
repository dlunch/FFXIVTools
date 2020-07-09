use alloc::{str, vec};

use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use sqpack_reader::{ExtractedFileProviderWeb, Package, SqPackReaderExtractedFile};

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        spawn_local(async {
            let provider = ExtractedFileProviderWeb::new("https://ffxiv-data3.dlunch.net");
            let package = SqPackReaderExtractedFile::new(provider);

            let exl = package.read_file("exd/root.exl").await.unwrap();
            let exl_str = str::from_utf8(&exl).unwrap();
            console::log_1(&exl_str.into());
        });
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Hello world!" }</p>
        }
    }
}
