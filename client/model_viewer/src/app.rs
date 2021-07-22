use core::cell::{Ref, RefCell};

use winit::event_loop::EventLoop;
use yew::prelude::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

pub struct App {
    canvas: NodeRef,
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
    pub fn content(&self) -> Ref<'_, Content> {
        let content_ref = self.content.borrow();

        Ref::map(content_ref, |x| x.as_ref().unwrap())
    }

    pub fn start(&self, event_loop: &EventLoop<()>) {
        self.content.replace(Some(Content::new(event_loop)));
    }
}
