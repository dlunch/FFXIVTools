use std::sync::Arc;

use log::debug;
use nalgebra::Point3;
use tokio::runtime::Runtime;
use winit::{
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use ffxiv_model::{Character, ShaderHolder};
use renderer::{Camera, Renderer, Scene};
use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

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

    let mut app = rt.block_on(async {
        let mut app = App::new(&window).await.unwrap();
        app.add_character().await.unwrap();

        app
    });

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
                rt.block_on(async { app.render().await });
            }
            _ => {}
        }
    });
}

struct App<'a> {
    renderer: Renderer,
    shader_holder: Arc<ShaderHolder>,
    package: SqPackReaderExtractedFile,
    scene: Scene<'a>,
}

impl<'a> App<'a> {
    pub async fn new(window: &Window) -> Result<App<'a>> {
        let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/", |current, total| {
            debug!("{}/{}", current, total)
        });
        let package = SqPackReaderExtractedFile::new(provider)?;

        let size = window.inner_size();
        let renderer = Renderer::new(window, size.width, size.height).await;
        let shader_holder = Arc::new(ShaderHolder::new(&renderer));

        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        let scene = Scene::new(camera);

        Ok(Self {
            renderer,
            shader_holder,
            package,
            scene,
        })
    }

    pub async fn add_character(&mut self) -> Result<()> {
        let mut character = Character::new(self.shader_holder.clone());
        character.add_equipment(&self.renderer, &self.package).await?;

        self.scene.add(character);

        Ok(())
    }

    pub async fn render(&mut self) {
        self.renderer.render(&self.scene).await;
    }
}
