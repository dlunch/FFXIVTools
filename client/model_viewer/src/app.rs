use core::cell::RefCell;

use winit::window::Window;
use yew::prelude::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub struct App {
    pub canvas: NodeRef,
    content: RefCell<Option<Content>>,
}

use super::content::Content;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            canvas: NodeRef::default(),
            content: RefCell::new(None),
        }
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

impl App {
    pub fn request_redraw(&self) {
        let content = self.content.borrow();
        if let Some(content) = content.as_ref() {
            content.request_redraw();
        }
    }

    pub fn redraw(&self) {
        let mut content = self.content.borrow_mut();
        if let Some(content) = content.as_mut() {
            content.redraw();
        }
    }

    pub async fn start(&self, window: Window) {
        self.content.replace(Some(Content::new(window).await));
    }
}
