#![type_length_limit = "2097152"]

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use async_std::fs;
use async_std::task;
use hashbrown::HashMap;
use log::debug;
use nalgebra::Point3;
use once_cell::sync::OnceCell;
use winit::{
    dpi::LogicalSize,
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};
use renderer::{Camera, Renderer, Scene, WindowRenderTarget};
use sqpack::{Result, SqPackPackage};
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};

// task::spawn requires 'static lifetime.
static mut APP: OnceCell<App> = OnceCell::new();

fn main() {
    let _ = pretty_env_logger::init_timed();

    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test").with_inner_size(LogicalSize::new(1920, 1080));
    let window = builder.build(&event_loop).unwrap();

    task::block_on(async {
        unsafe {
            let _ = APP.set(App::new(&window).await);
            APP.get_mut().unwrap().add_character().await.unwrap();
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
            event::Event::RedrawRequested(_) => {
                task::block_on(async move {
                    let app = unsafe { APP.get_mut().unwrap() };
                    app.render().await;
                });
            }
            _ => {}
        }
    });
}

struct App<'a> {
    renderer: Renderer,
    render_target: WindowRenderTarget,
    context: Context,
    package: Arc<BatchedPackage>,
    scene: Scene<'a>,
}

impl<'a> App<'a> {
    pub async fn new(window: &Window) -> App<'a> {
        #[cfg(unix)]
        let path = "/mnt/d/Games/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack";
        #[cfg(windows)]
        let path = "D:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack";

        let package = if fs::metadata(path).await.is_ok() {
            BatchedPackage::new(SqPackPackage::new(&Path::new(path)).unwrap())
        } else {
            let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/all/", |current, total| {
                debug!("{}/{}", current, total)
            });
            BatchedPackage::new(SqPackReaderExtractedFile::new(provider))
        };
        let package = Arc::new(package);

        // TODO We can't put add_character in task::spawn (rust issue #64650), so BatchedPackage::poll can't be in app.update() or somewhere.
        let package2 = package.clone();
        task::spawn_local(async move {
            loop {
                package2.poll().await.unwrap();
                task::sleep(Duration::from_millis(16)).await;
            }
        });

        let size = window.inner_size();
        let renderer = Renderer::new().await;
        let render_target = WindowRenderTarget::new(&renderer, window, size.width, size.height);
        let context = Context::new(&renderer, &*package).await.unwrap();

        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        let scene = Scene::new(camera);

        Self {
            renderer,
            render_target,
            context,
            package,
            scene,
        }
    }

    pub async fn add_character(&'a mut self) -> Result<()> {
        let mut equipments = HashMap::new();
        equipments.insert(ModelPart::Met, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Top, Equipment::new(6016, 1, 20));
        equipments.insert(ModelPart::Glv, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Dwn, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Sho, Equipment::new(6016, 1, 0));

        let customization = Customization::new(BodyId::AuRaFemale, 1, 1, 1, 1, 1);
        let character = Character::new(&self.renderer, &*self.package, &self.context, customization, equipments).await?;

        self.scene.add(character);
        Ok(())
    }

    pub async fn render(&mut self) {
        self.package.poll().await.unwrap();
        self.renderer.render(&self.scene, &mut self.render_target);
    }
}
