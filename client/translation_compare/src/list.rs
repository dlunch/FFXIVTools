use alloc::collections::BTreeMap;

use yew::prelude::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct List {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub data: BTreeMap<u32, Vec<String>>,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            <ul>
                {
                    self
                        .props
                        .data
                        .iter()
                        .map(|(k, v)| html! { <li>{ format!("{}: {}", k, v.join(", ")) }</li> })
                        .collect::<Html>()
                }
            </ul>
        }
    }
}
