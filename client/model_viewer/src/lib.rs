extern crate alloc;

mod app;
mod content;

use wasm_bindgen::prelude::wasm_bindgen;
use winit::{
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    let event_loop = EventLoop::new();

    yew::initialize();
    let link = yew::App::<app::App>::new().mount_to_body();
    link.get_component().unwrap().start(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        let app = link.get_component().unwrap();
        *control_flow = ControlFlow::Poll;

        match event {
            event::Event::MainEventsCleared => app.content().request_redraw(),
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            event::Event::RedrawRequested(_) => {
                app.content().redraw();
            }
            _ => {}
        }
    });
}
