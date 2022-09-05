use yew::prelude::{html, Component, Context, Html};

pub struct Content {}

pub enum Msg {}

impl Component for Content {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <p> { "content" } </p>
            </div>
        }
    }
}
