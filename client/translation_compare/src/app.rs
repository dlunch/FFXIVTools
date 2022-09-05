#![allow(dead_code)] // warning: associated function `build` is never used on #[derive(Properties)]

use alloc::collections::BTreeMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::{html, Component, Context, Html, Properties};

use common::{regions, Region, WasmPackage};
use ffxiv_ex::{Action, BNpcName, ClassJob, CraftAction, ENpcResident, Item, NamedExRow, PlaceName, Quest, WrappedEx};
use sqpack::Result;

use crate::list::List;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub base_url: String,
}

pub struct App {
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
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: BTreeMap::new(),
            progress: (0, 0),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Load(x) => {
                self.load(ctx, x);
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

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                <button onclick={ctx.link().callback(move |_| Msg::Load(x))}>{ x }</button>
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
                <List data={self.data.clone()}>
                </List>
            </div>
        }
    }
}

impl App {
    fn load(&self, ctx: &Context<Self>, name: &'static str) {
        let progress_callback = ctx.link().callback(Msg::Progress);
        let ready_callback = ctx.link().callback(Msg::OnDataReady);

        let base_url = ctx.props().base_url.clone();
        spawn_local(async move {
            let regions = regions();
            let mut result = BTreeMap::new();

            progress_callback.emit((0, regions.len()));
            for (i, region) in regions.iter().enumerate() {
                progress_callback.emit((i, regions.len()));
                let names = match name {
                    "classjob" => Self::read_names::<ClassJob>(region, &base_url).await,
                    "item" => Self::read_names::<Item>(region, &base_url).await,
                    "action" => Self::read_names::<Action>(region, &base_url).await,
                    "craftaction" => Self::read_names::<CraftAction>(region, &base_url).await,
                    "bnpcname" => Self::read_names::<BNpcName>(region, &base_url).await,
                    "enpcresident" => Self::read_names::<ENpcResident>(region, &base_url).await,
                    "quest" => Self::read_names::<Quest>(region, &base_url).await,
                    "placename" => Self::read_names::<PlaceName>(region, &base_url).await,
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

    async fn read_names<'a, T: NamedExRow<'static> + 'static>(region: &Region, base_url: &str) -> Result<BTreeMap<u32, Vec<String>>> {
        let package = WasmPackage::new(region, base_url).await;

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
