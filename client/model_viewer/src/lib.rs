extern crate alloc;

use alloc::rc::Rc;

mod app;
mod content;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use winit::{
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Trace).unwrap();

    yew::initialize();
    let scope = Rc::new(yew::App::<app::App>::new().mount_to_body());

    let event_loop = EventLoop::new();
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::HtmlCanvasElement;
        use winit::platform::web::WindowBuilderExtWebSys;

        builder = builder.with_canvas(link.get_component().unwrap().canvas.cast::<HtmlCanvasElement>());
    }

    let window = builder.build(&event_loop).unwrap();

    let scope2 = scope.clone();
    spawn_local(async move {
        scope2.get_component().unwrap().start(window).await;
    });

    event_loop.run(move |event, _, control_flow| {
        let app = scope.get_component().unwrap();
        *control_flow = ControlFlow::Poll;

        match event {
            event::Event::MainEventsCleared => app.request_redraw(),
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
                app.redraw();
            }
            _ => {}
        }
    });
}
