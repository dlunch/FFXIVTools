use std::f32::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use hashbrown::HashMap;
use log::debug;
use nalgebra::Point3;
use tokio::{fs, task, time};
use winit::{
    dpi::PhysicalSize,
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use eng::render::{ArcballCameraController, PerspectiveCamera, Renderer, Scene};
use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};
use sqpack::{Result, SqPackPackage};
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test").with_inner_size(PhysicalSize::new(1920, 1080));
    let window = builder.build(&event_loop).unwrap();

    let mut app = App::new(&window).await;
    app.add_character().await.unwrap();

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
                app.render();
            }
            _ => {}
        }
    });
}

struct App {
    renderer: Renderer,
    context: Context,
    package: Arc<BatchedPackage>,
    scene: Scene,
    camera: PerspectiveCamera<ArcballCameraController>,
}

impl App {
    pub async fn new(window: &Window) -> App {
        #[cfg(unix)]
        let path = "/mnt/d/Games/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack";
        #[cfg(windows)]
        let path = "D:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack";

        let package = if fs::metadata(path).await.is_ok() {
            BatchedPackage::new(SqPackPackage::new(Path::new(path)).unwrap())
        } else {
            let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/all/", |current, total| {
                debug!("{}/{}", current, total)
            });
            BatchedPackage::new(SqPackReaderExtractedFile::new(provider))
        };
        let package = Arc::new(package);

        // TODO We can't put add_character in task::spawn (rust issue #64650), so BatchedPackage::poll can't be in app.update() or somewhere.
        let package2 = package.clone();
        task::spawn(async move {
            loop {
                package2.poll().await.unwrap();
                time::sleep(Duration::from_millis(16)).await;
            }
        });

        let size = window.inner_size();
        let renderer = Renderer::new(window, size.width, size.height).await;
        let context = Context::new(&renderer, &*package).await.unwrap();

        let controller = ArcballCameraController::new(Point3::new(0.0, 0.8, 0.0), 2.5);
        let camera = PerspectiveCamera::new(45.0 * PI / 180.0, size.width as f32 / size.height as f32, 0.1, 100.0, controller);
        let scene = Scene::new();

        Self {
            renderer,
            context,
            package,
            scene,
            camera,
        }
    }

    pub async fn add_character(&mut self) -> Result<()> {
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

    pub fn render(&mut self) {
        self.renderer.render(&self.camera, &self.scene);
    }
}
