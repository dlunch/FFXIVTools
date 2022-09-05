use alloc::collections::BTreeMap;

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct List {}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub data: BTreeMap<u32, Vec<String>>,
}

impl Component for List {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul>
                {
                    ctx
                        .props()
                        .data
                        .iter()
                        .map(|(k, v)| html! { <li>{ format!("{}: {}", k, v.join(", ")) }</li> })
                        .collect::<Html>()
                }
            </ul>
        }
    }
}
