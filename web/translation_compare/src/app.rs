use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use ffxiv_exd::{ClassJob, NamedExRow, WrappedEx};
use ffxiv_parser::Language;
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct Exs {
    class_job: WrappedEx<'static, ClassJob<'static>>,
}

pub struct App {
    exs: Option<Exs>,
}

pub enum Msg {
    ExsReady(Exs),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::ExsReady);

        spawn_local(async move {
            let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
            let package = SqPackReaderExtractedFile::new(provider);

            let class_job = WrappedEx::<ClassJob>::new(&package).await.unwrap();

            callback.emit(Exs { class_job });
        });

        App { exs: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ExsReady(x) => {
                self.exs = Some(x);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if let Some(x) = &self.exs {
            let items = x
                .class_job
                .all(Language::English)
                .unwrap()
                .iter()
                .map(|(_, value)| html! { <li>{ value.name() }</li> })
                .collect::<Html>();
            html! {
                <ul>
                    { items }
                </ul>
            }
        } else {
            html! {}
        }
    }
}
