use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::treeview::{TreeView, TreeViewData, TreeViewItem};

#[derive(Clone, PartialEq)]
struct Item {
    text: String,
}

pub struct App {
    tree_data: Vec<TreeViewItem<Item>>,
}

impl TreeViewData for Item {
    fn render(&self) -> Html {
        html! {
            <> { &self.text } </>
        }
    }
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let test_data = vec![TreeViewItem::new(
            Item { text: "root".into() },
            vec![
                TreeViewItem::new(
                    Item { text: "child1".into() },
                    vec![
                        TreeViewItem::new(
                            Item { text: "child11".into() },
                            vec![TreeViewItem::new(Item { text: "child111".into() }, Vec::new())],
                        ),
                        TreeViewItem::new(Item { text: "child12".into() }, Vec::new()),
                    ],
                ),
                TreeViewItem::new(
                    Item { text: "child2".into() },
                    vec![TreeViewItem::new(Item { text: "child21".into() }, Vec::new())],
                ),
                TreeViewItem::new(Item { text: "child3".into() }, Vec::new()),
            ],
        )];

        App { tree_data: test_data }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <TreeView<Item> data = &self.tree_data />
        }
    }
}
