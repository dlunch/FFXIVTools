use alloc::collections::BTreeMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use common::{regions, Region, WasmPackage};
use ffxiv_ex::{Action, BNpcName, ClassJob, CraftAction, ENpcResident, Item, NamedExRow, PlaceName, Quest, WrappedEx};
use sqpack::Result;

use crate::list::List;

pub struct App {
    link: ComponentLink<Self>,
    data: BTreeMap<u32, Vec<String>>,
    progress: (usize, usize),
}

pub enum Msg {
    Progress((usize, usize)),
    Load(&'static str),
    OnDataReady(BTreeMap<u32, Vec<String>>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: BTreeMap::new(),
            progress: (0, 0),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Load(x) => {
                self.load(x);
                true
            }
            Msg::OnDataReady(x) => {
                self.data = x;
                true
            }
            Msg::Progress(x) => {
                self.progress = x;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let buttons = [
            "classjob",
            "item",
            "action",
            "craftaction",
            "bnpcname",
            "enpcresident",
            "quest",
            "placename",
        ]
        .iter()
        .map(|x| {
            html! {
                <button onclick=self.link.callback(move |_| Msg::Load(x))>{ x }</button>
            }
        })
        .collect::<Html>();

        html! {
            <div>
                <div>
                    { buttons }
                </div>
                <p>
                {
                    if self.progress.0 != self.progress.1 {
                        html! { format!("Loading... {}/{}", self.progress.0, self.progress.1) }
                    }
                    else {
                        html! {}
                    }
                }
                </p>
                <List data = &self.data>
                </List>
            </div>
        }
    }
}

impl App {
    fn load(&self, name: &'static str) {
        let progress_callback = self.link.callback(Msg::Progress);
        let ready_callback = self.link.callback(Msg::OnDataReady);

        spawn_local(async move {
            let regions = regions();
            let mut result = BTreeMap::new();

            progress_callback.emit((0, regions.len()));
            for (i, region) in regions.iter().enumerate() {
                progress_callback.emit((i, regions.len()));
                let names = match name {
                    "classjob" => Self::read_names::<ClassJob>(region).await,
                    "item" => Self::read_names::<Item>(region).await,
                    "action" => Self::read_names::<Action>(region).await,
                    "craftaction" => Self::read_names::<CraftAction>(region).await,
                    "bnpcname" => Self::read_names::<BNpcName>(region).await,
                    "enpcresident" => Self::read_names::<ENpcResident>(region).await,
                    "quest" => Self::read_names::<Quest>(region).await,
                    "placename" => Self::read_names::<PlaceName>(region).await,
                    _ => panic!(),
                }
                .unwrap();

                for (k, mut v) in names {
                    result.entry(k).or_insert_with(Vec::new).append(&mut v);
                }
            }
            progress_callback.emit((regions.len(), regions.len()));

            ready_callback.emit(result);
        });
    }

    async fn read_names<'a, T: NamedExRow<'static> + 'static>(region: &Region) -> Result<BTreeMap<u32, Vec<String>>> {
        let package = WasmPackage::new(region);

        let wrapped_ex = WrappedEx::<T>::new(&package).await?;
        // TODO do we really require unsafe here??
        let wrapped_ex_ref: &WrappedEx<T> = unsafe { core::mem::transmute(&wrapped_ex) };

        let mut result = BTreeMap::<u32, Vec<_>>::new();

        for language in &region.languages {
            let all = wrapped_ex_ref.all(*language).unwrap();

            for (k, v) in all {
                let name = v.name();
                if !name.is_empty() {
                    result.entry(k).or_insert_with(Vec::new).push(name);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_read_name() {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack"), log::LevelFilter::Debug)
            .try_init();

        let region = &regions()[0];
        let _ = App::read_names::<Item>(region).await.unwrap();
    }
}
