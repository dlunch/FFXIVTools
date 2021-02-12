use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ContentComponent {}

pub enum Msg {}

impl Component for ContentComponent {
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
            <div class="content-component">
                <p> { "content" } </p>
            </div>
        }
    }
}
