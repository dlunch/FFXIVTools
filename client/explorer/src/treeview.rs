use std::collections::BTreeSet;
use std::sync::atomic::{AtomicUsize, Ordering};

use yew::prelude::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub trait TreeViewData: std::clone::Clone + std::cmp::PartialEq {
    fn render(&self) -> Html;
}

#[derive(Clone, PartialEq)]
pub struct TreeViewItem<T: TreeViewData> {
    id: usize, // auto generated

    pub data: T,
    pub children: Vec<TreeViewItem<T>>,
}

static mut TREE_ID: AtomicUsize = AtomicUsize::new(0);

impl<T: TreeViewData> TreeViewItem<T> {
    pub fn new(data: T, children: Vec<TreeViewItem<T>>) -> Self {
        let id = unsafe { TREE_ID.fetch_add(1, Ordering::SeqCst) };

        Self { id, data, children }
    }
}

pub enum Msg {
    TreeItemClick(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: TreeViewData> {
    pub data: Vec<TreeViewItem<T>>,
}

pub struct TreeView<T: TreeViewData + 'static> {
    link: ComponentLink<Self>,
    props: Props<T>,
    shown_items: BTreeSet<usize>,
}

impl<T: TreeViewData + 'static> Component for TreeView<T> {
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            shown_items: BTreeSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TreeItemClick(x) => {
                self.toggle_item(x);

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
        html! {
            <ul class="tree-view">
            {
                 self
                    .props
                    .data
                    .iter()
                    .map(|x| self.render_item(x))
                    .collect::<Html>()
            }
            </ul>
        }
    }
}

impl<T: TreeViewData> TreeView<T> {
    fn render_item(&self, item: &TreeViewItem<T>) -> Html {
        let id = item.id;

        html! {
            <li>
                <span onclick=self.link.callback(move |_| Msg::TreeItemClick(id))>{ item.data.render() }</span>
                {
                    if self.shown_items.contains(&item.id) {
                        html! { <TreeView<T> data=item.children.clone() /> }
                    } else {
                        html! {}
                    }
                }
            </li>
        }
    }

    fn toggle_item(&mut self, item_id: usize) {
        if self.shown_items.contains(&item_id) {
            self.shown_items.remove(&item_id);
        } else {
            self.shown_items.insert(item_id);
        }
    }
}
