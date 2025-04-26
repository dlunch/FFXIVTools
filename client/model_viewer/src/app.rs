use core::cell::RefCell;

use winit::window::Window;
use yew::prelude::{Component, Context, Html, NodeRef, html};

pub struct App {
    pub canvas: NodeRef,
    content: RefCell<Option<Content>>,
}

use super::content::Content;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            canvas: NodeRef::default(),
            content: RefCell::new(None),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas.clone()} />
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
