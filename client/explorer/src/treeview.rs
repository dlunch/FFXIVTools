use yew::prelude::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct TreeView<T: std::clone::Clone> {
    props: Props<T>,
}

#[derive(Clone, PartialEq)]
pub struct TreeViewItem<T> {
    pub data: T,
    pub children: Vec<TreeViewItem<T>>,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: std::clone::Clone> {
    pub data: TreeViewItem<T>,
}

impl<T: std::clone::Clone + std::cmp::PartialEq + 'static> Component for TreeView<T> {
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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
            <p>{ "Hello world!" }</p>
        }
    }
}
