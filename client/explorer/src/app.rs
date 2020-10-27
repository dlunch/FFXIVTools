use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::treeview::{TreeView, TreeViewItem};

#[derive(Clone, PartialEq)]
struct TreeItem {
    text: String,
}

pub struct App {
    tree_data: TreeViewItem<TreeItem>,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            tree_data: TreeViewItem {
                data: TreeItem { text: "".into() },
                children: Vec::new(),
            },
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <TreeView<TreeItem> data = &self.tree_data />
        }
    }
}
