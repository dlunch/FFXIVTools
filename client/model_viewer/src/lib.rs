extern crate alloc;

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

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    {
        console_error_panic_hook::set_once();

        fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .level_for("wgpu", log::LevelFilter::Info)
            .chain(fern::Output::call(console_log::log))
            .apply()
            .unwrap();
    }

    let app = yew::start_app::<app::App>();

    let event_loop = EventLoop::new();
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::HtmlCanvasElement;
        use winit::platform::web::WindowBuilderExtWebSys;

        builder = builder.with_canvas(app.get_component().unwrap().canvas.cast::<HtmlCanvasElement>());
    }

    let window = builder.build(&event_loop).unwrap();

    let app2 = app.clone();
    spawn_local(async move {
        app2.get_component().unwrap().start(window).await;
    });

    event_loop.run(move |event, _, control_flow| {
        let app = app.get_component().unwrap();
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
