use ffxiv_renderer::FFXIVRenderer;

fn main() {
    use winit::{
        event,
        event::WindowEvent,
        event_loop::{ControlFlow, EventLoop},
    };

    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test");
    let window = builder.build(&event_loop).unwrap();

    let mut renderer = FFXIVRenderer::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            event::Event::MainEventsCleared => window.request_redraw(),
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
                renderer.redraw();
            }
            _ => {}
        }
    });
}
