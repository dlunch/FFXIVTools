use yew::prelude::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct List {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data: Option<Vec<(u32, String)>>,
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
        let data = if let Some(x) = &self.props.data {
            x.iter().map(|(_, v)| html! { <li>{ v }</li> }).collect::<Html>()
        } else {
            html! {}
        };
        html! {
            <ul>
                { data }
            </ul>
        }
    }
}
