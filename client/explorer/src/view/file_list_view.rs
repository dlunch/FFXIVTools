use yew::prelude::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use wasm_bindgen_futures::spawn_local;

use super::tree_view::{TreeView, TreeViewData, TreeViewItem};
use crate::context::Context;

#[derive(Clone, PartialEq)]
pub struct Item {
    text: String,
}

impl TreeViewData for Item {
    fn render(&self) -> Html {
        html! {
            <> { &self.text } </>
        }
    }
}

pub enum Msg {
    FetchTreeViewData((String, Callback<Vec<TreeViewItem<String, Item>>>)),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub file_select_callback: Callback<String>,
}

pub struct FileListView {
    link: ComponentLink<Self>,
}

impl Component for FileListView {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchTreeViewData((key, callback)) => {
                spawn_local(async move {
                    let context = Context::get();
                    let files = context.file_list.get_files(&key).await.unwrap();

                    let result = files
                        .into_iter()
                        .map(|x| {
                            let new_key = if key.is_empty() { x.clone() } else { format!("{}/{}", key, x) };

                            TreeViewItem {
                                key: new_key,
                                value: Item { text: x },
                            }
                        })
                        .collect::<Vec<_>>();

                    callback.emit(result);
                });

                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <TreeView<String, Item> item_key="" data_request_callback=self.link.callback(move |x| Msg::FetchTreeViewData(x)) />
        }
    }
}
