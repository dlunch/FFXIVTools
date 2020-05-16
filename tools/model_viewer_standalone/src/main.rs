use std::sync::Arc;

use log::debug;
use nalgebra::Point3;
use tokio::runtime::Runtime;
use winit::{
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

use ffxiv_model::{Character, ShaderHolder};
use renderer::{Camera, Renderer, Scene};
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

fn main() {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
        .filter(Some("model_viewer_standalone"), log::LevelFilter::Debug)
        .try_init();

    let mut rt = Runtime::new().unwrap();
    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test");
    let window = builder.build(&event_loop).unwrap();

    let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/", |current, total| {
        debug!("{}/{}", current, total)
    });
    let pack = SqPackReaderExtractedFile::new(provider).unwrap();

    let (mut renderer, shader_holder) = rt.block_on(async {
        let size = window.inner_size();
        let renderer = Renderer::new(&window, size.width, size.height).await;
        let shader_holder = ShaderHolder::new(&renderer);

        (renderer, shader_holder)
    });

    let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
    let mut scene = Scene::new(camera);
    let shader_holder = Arc::new(shader_holder);
    let mut character = Character::new(shader_holder.clone());
    rt.block_on(async {
        character.add_equipment(&renderer, &pack).await.unwrap();
    });
    scene.add(character);

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
                rt.block_on(async { renderer.render(&mut scene).await });
            }
            _ => {}
        }
    });
}
