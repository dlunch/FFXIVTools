use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use log::debug;

use crate::view::content_view::ContentView;
use crate::view::file_list_view::FileListView;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    FileSelected(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FileSelected(x) => {
                debug!("{}", x);

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <FileListView file_select_callback=self.link.callback(|x| Msg::FileSelected(x)) />
                <ContentView />
            </div>
        }
    }
}
