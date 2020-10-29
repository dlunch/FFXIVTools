use std::boxed::Box;
use std::collections::HashSet;

use yew::prelude::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub trait TreeViewData: std::clone::Clone + std::cmp::PartialEq {
    fn render(&self) -> Html;
}

#[derive(Clone, PartialEq)]
pub struct TreeViewItem<K: std::clone::Clone + std::cmp::PartialEq + std::hash::Hash, V: TreeViewData> {
    pub key: K,
    pub value: V,
}

pub enum Msg<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeViewData> {
    TreeItemClick(K),
    Children(Vec<TreeViewItem<K, V>>),
}

type DataRequestCallback<K, V> = Callback<(K, Callback<Vec<TreeViewItem<K, V>>>)>;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeViewData> {
    pub item_key: K,
    pub data_request_callback: DataRequestCallback<K, V>,
}

pub struct TreeView<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash + 'static, V: TreeViewData + 'static> {
    link: ComponentLink<Self>,
    props: Props<K, V>,
    shown_items: HashSet<K>,
    data: Option<Vec<TreeViewItem<K, V>>>,
}

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash + 'static, V: TreeViewData + 'static> Component for TreeView<K, V> {
    type Message = Msg<K, V>;
    type Properties = Props<K, V>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            shown_items: HashSet::new(),
            data: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TreeItemClick(x) => {
                self.toggle_item(x);

                true
            }
            Msg::Children(x) => {
                self.data = Some(x);

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if let Some(x) = &self.data {
            let items = x.iter().map(|x| self.render_item(x)).collect::<Html>();
            html! {
                <ul class="tree-view">{ items }</ul>
            }
        } else {
            self.props
                .data_request_callback
                .emit((self.props.item_key.clone(), self.link.callback(Msg::Children)));

            html! {
                <div>{ "Loading..." }</div>
            }
        }
    }
}

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeViewData> TreeView<K, V> {
    fn render_item(&self, item: &TreeViewItem<K, V>) -> Html {
        let expanded = self.shown_items.contains(&item.key);

        let children = if expanded {
            html! { <TreeView<K, V> item_key=item.key.clone() data_request_callback=self.props.data_request_callback.clone() /> }
        } else {
            html! {}
        };

        let key = Box::new(item.key.clone());
        let callback = self.link.callback(move |_| Msg::TreeItemClick(key.as_ref().clone()));
        html! {
            <li class={ if expanded { "expanded" } else { "" } }>
                <span onclick=callback>{ item.value.render() }</span>
                { children }
            </li>
        }
    }

    fn toggle_item(&mut self, key: K) {
        if self.shown_items.contains(&key) {
            self.shown_items.remove(&key);
        } else {
            self.shown_items.insert(key);
        }
    }
}
