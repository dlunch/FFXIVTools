use yew::prelude::{Component, Context, Html, html};

use log::debug;

use crate::components::content::Content;
use crate::components::file_list::FileList;

pub struct App {}

pub enum Msg {
    FileSelected(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileSelected(x) => {
                debug!("{}", x);

                true
            }
        }
    }

    #[allow(clippy::let_unit_value)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <FileList file_select_callback={ctx.link().callback(Msg::FileSelected)} />
                <Content />
            </div>
        }
    }
}
