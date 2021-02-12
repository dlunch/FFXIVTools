use std::boxed::Box;
use std::collections::HashSet;

use yew::prelude::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub trait TreeData: std::clone::Clone + std::cmp::PartialEq {
    fn render(&self) -> Html;
}

#[derive(Clone, PartialEq)]
pub struct TreeItem<K: std::clone::Clone + std::cmp::PartialEq + std::hash::Hash, V: TreeData> {
    pub key: K,
    pub value: V,
}

pub enum Msg<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeData> {
    TreeItemClick(K),
    Children(Vec<TreeItem<K, V>>),
}

type DataRequestCallback<K, V> = Callback<(K, Callback<Vec<TreeItem<K, V>>>)>;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeData> {
    pub item_key: K,
    pub data_request_callback: DataRequestCallback<K, V>,
}

pub struct Tree<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash + 'static, V: TreeData + 'static> {
    link: ComponentLink<Self>,
    props: Props<K, V>,
    shown_items: HashSet<K>,
    data: Option<Vec<TreeItem<K, V>>>,
}

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash + 'static, V: TreeData + 'static> Component for Tree<K, V> {
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

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeData> Tree<K, V> {
    fn render_item(&self, item: &TreeItem<K, V>) -> Html {
        let expanded = self.shown_items.contains(&item.key);

        let children = if expanded {
            html! { <Tree<K, V> item_key=item.key.clone() data_request_callback=self.props.data_request_callback.clone() /> }
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
