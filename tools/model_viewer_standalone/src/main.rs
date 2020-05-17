use std::path::Path;
use std::sync::Arc;

use log::debug;
use nalgebra::Point3;
use once_cell::sync::OnceCell;
use tokio::fs;
use tokio::sync::Notify;
use winit::{
    dpi::LogicalSize,
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use ffxiv_model::{BodyId, Character, ModelPart, ShaderHolder};
use renderer::{Camera, Renderer, Scene};
use sqpack_reader::{ExtractedFileProviderWeb, Package, Result, SqPackReader, SqPackReaderExtractedFile};

static mut APP: OnceCell<App> = OnceCell::new();

#[tokio::main]
async fn main() {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
        .filter(Some("model_viewer_standalone"), log::LevelFilter::Debug)
        .try_init();

    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test").with_inner_size(LogicalSize::new(1920, 1080));
    let window = builder.build(&event_loop).unwrap();

    unsafe {
        let _ = APP.set(App::new(&window).await);
        APP.get_mut().unwrap().add_character().await.unwrap();
    }

    let notifier = Arc::new(Notify::new());
    let notify_read = notifier.clone();

    tokio::spawn(async move {
        let app = unsafe { APP.get_mut().unwrap() };
        loop {
            notify_read.notified().await;
            app.render().await;
        }
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
            event::Event::RedrawRequested(_) => notifier.notify(),
            _ => {}
        }
    });
}

struct App<'a> {
    renderer: Renderer,
    shader_holder: ShaderHolder,
    package: Box<dyn Package>,
    scene: Scene<'a>,
}

impl<'a> App<'a> {
    pub async fn new(window: &Window) -> App<'a> {
        #[cfg(unix)]
        let path = "/mnt/d/Games/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack";
        #[cfg(windows)]
        let path = "D:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack";

        let package: Box<dyn Package> = if fs::metadata(path).await.is_ok() {
            Box::new(SqPackReader::new(&Path::new(path)).unwrap())
        } else {
            let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/", |current, total| {
                debug!("{}/{}", current, total)
            });
            Box::new(SqPackReaderExtractedFile::new(provider))
        };

        let size = window.inner_size();
        let renderer = Renderer::new(window, size.width, size.height).await;
        let shader_holder = ShaderHolder::new(&renderer);

        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        let scene = Scene::new(camera);

        Self {
            renderer,
            shader_holder,
            package,
            scene,
        }
    }

    pub async fn add_character(&'a mut self) -> Result<()> {
        let mut character = Character::new(&self.renderer, &*self.package, &self.shader_holder, BodyId::MidlanderFemale, 1, 1);

        character.add_equipment(6016, 1, ModelPart::Met).await?;
        character.add_equipment(6016, 1, ModelPart::Top).await?;
        character.add_equipment(6016, 1, ModelPart::Glv).await?;
        character.add_equipment(6016, 1, ModelPart::Dwn).await?;
        character.add_equipment(6016, 1, ModelPart::Sho).await?;

        self.scene.add(character);
        Ok(())
    }

    pub async fn render(&mut self) {
        self.renderer.render(&self.scene).await;
    }
}
