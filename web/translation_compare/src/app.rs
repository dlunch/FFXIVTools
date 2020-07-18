use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use ffxiv_exd::{ClassJob, NamedExRow, WrappedEx};
use ffxiv_parser::Language;
use sqpack_reader::{ExtractedFileProviderWeb, Package, Result, SqPackReaderExtractedFile};

use crate::list::List;

pub struct App {
    link: ComponentLink<Self>,
    data: Option<Vec<(u32, String)>>,
}

pub enum Msg {
    OnDisplay(&'static str),
    OnDataReady(Vec<(u32, String)>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnDisplay(x) => {
                self.display_result(x);
                true
            }
            Msg::OnDataReady(x) => {
                self.data = Some(x);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let buttons = ["classjob", "item", "action", "craftaction", "enemy", "npc", "quest", "place"]
            .iter()
            .map(|x| {
                html! {
                    <button onclick=self.link.callback(move |_| Msg::OnDisplay(x))>{ x }</button>
                }
            })
            .collect::<Html>();

        html! {
            <div>
                <span>
                    { buttons }
                </span>
                <List data = &self.data>
                </List>
            </div>
        }
    }
}

impl App {
    fn display_result(&self, name: &'static str) {
        let callback = self.link.callback(Msg::OnDataReady);

        spawn_local(async move {
            let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
            let package = SqPackReaderExtractedFile::new(provider);

            let names = match name {
                "classjob" => Self::read_names::<ClassJob>(&package, Language::English).await.unwrap(),
                _ => panic!(),
            };

            callback.emit(names);
        });
    }

    async fn read_names<'a, T: NamedExRow<'static> + 'static>(package: &dyn Package, language: Language) -> Result<Vec<(u32, String)>> {
        let wrapped_ex = WrappedEx::<T>::new(package).await?;

        // TODO do we really require unsafe here??
        let wrapped_ex_ref: &WrappedEx<T> = unsafe { core::mem::transmute(&wrapped_ex) };
        let all = wrapped_ex_ref.all(language).unwrap();

        Ok(all.map(|(k, v)| (k, v.name())).collect::<Vec<_>>())
    }
}
