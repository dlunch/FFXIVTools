use yew::prelude::{Callback, Component, Context, Html, Properties, html};

use wasm_bindgen_futures::spawn_local;

use super::tree::{Tree, TreeData, TreeItem};
use crate::context::AppContext;

#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    text: String,
}

impl TreeData for Item {
    fn render(&self) -> Html {
        html! {
            <> { &self.text } </>
        }
    }
}

pub enum Msg {
    FetchTreeData((String, Callback<Vec<TreeItem<String, Item>>>)),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub file_select_callback: Callback<String>,
}

pub struct FileList {}

impl Component for FileList {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchTreeData((key, callback)) => {
                spawn_local(async move {
                    let context = AppContext::get();
                    let files = context.file_list.get_files(&key).await.unwrap();

                    let result = files
                        .into_iter()
                        .map(|x| {
                            let new_key = if key.is_empty() { x.clone() } else { format!("{key}/{x}") };

                            TreeItem {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="file-list-component">
                <Tree<String, Item> item_key="" data_request_callback={ctx.link().callback(Msg::FetchTreeData)} />
            </div>
        }
    }
}
