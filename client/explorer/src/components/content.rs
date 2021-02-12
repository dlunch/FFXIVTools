use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Content {}

pub enum Msg {}

impl Component for Content {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="content">
                <p> { "content" } </p>
            </div>
        }
    }
}
