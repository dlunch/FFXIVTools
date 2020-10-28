use yew::prelude::{html, Callback, Component, ComponentLink, Html, ShouldRender};

use crate::treeview::{TreeView, TreeViewData, TreeViewItem};

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

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchTreeViewData((key, callback)) => {
                let new_key = format!("{}/key", key);
                let result = vec![TreeViewItem {
                    key: new_key.clone(),
                    value: Item { text: new_key },
                }];
                callback.emit(result);

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
