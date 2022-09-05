use std::boxed::Box;
use std::collections::HashSet;

use yew::prelude::{html, Callback, Component, Context, Html, Properties};

pub trait TreeData: std::clone::Clone + std::cmp::PartialEq {
    fn render(&self) -> Html;
}

#[derive(Clone, PartialEq, Eq)]
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
    shown_items: HashSet<K>,
    data: Option<Vec<TreeItem<K, V>>>,
}

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash + 'static, V: TreeData + 'static> Component for Tree<K, V> {
    type Message = Msg<K, V>;
    type Properties = Props<K, V>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            shown_items: HashSet::new(),
            data: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(x) = &self.data {
            let items = x.iter().map(|x| self.render_item(ctx, x)).collect::<Html>();
            html! {
                <ul class="tree-view">{ items }</ul>
            }
        } else {
            ctx.props()
                .data_request_callback
                .emit((ctx.props().item_key.clone(), ctx.link().callback(Msg::Children)));

            html! {
                <div>{ "Loading..." }</div>
            }
        }
    }
}

impl<K: std::clone::Clone + std::cmp::Eq + std::hash::Hash, V: TreeData> Tree<K, V> {
    fn render_item(&self, ctx: &Context<Self>, item: &TreeItem<K, V>) -> Html {
        let expanded = self.shown_items.contains(&item.key);

        let children = if expanded {
            html! { <Tree<K, V> item_key={item.key.clone()} data_request_callback={ctx.props().data_request_callback.clone()} /> }
        } else {
            html! {}
        };

        let key = Box::new(item.key.clone());
        let callback = ctx.link().callback(move |_| Msg::TreeItemClick(key.as_ref().clone()));
        html! {
            <li class={ if expanded { "expanded" } else { "" } }>
                <span onclick={callback}>{ item.value.render() }</span>
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
