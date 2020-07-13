use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use ffxiv_parser::ExList;
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct App {
    exl: Option<ExList>,
}

pub enum Msg {
    ExlReady(ExList),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::ExlReady);

        spawn_local(async move {
            let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
            let package = SqPackReaderExtractedFile::new(provider);

            let exl = ExList::new(&package).await.unwrap();

            callback.emit(exl);
        });

        App { exl: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ExlReady(x) => {
                self.exl = Some(x);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let exds = if let Some(x) = &self.exl {
            x.ex_names.iter().map(|x| html! { <li>{ x }</li> }).collect::<Html>()
        } else {
            html! {}
        };
        html! {
            <ul>
                { exds }
            </ul>
        }
    }
}
