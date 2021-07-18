use yew::prelude::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub struct App {
    pub canvas: NodeRef,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App { canvas: NodeRef::default() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <canvas ref=self.canvas.clone() />
        }
    }
}
