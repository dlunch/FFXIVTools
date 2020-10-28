use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct ContentView {}

pub enum Msg {}

impl Component for ContentView {
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
            <div class="content-view">
                <p> { "content" } </p>
            </div>
        }
    }
}
