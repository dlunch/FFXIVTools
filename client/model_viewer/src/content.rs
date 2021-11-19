use core::f32::consts::PI;

use hashbrown::HashMap;
use nalgebra::Point3;
use winit::window::Window;

use common::{regions, WasmPackage};
use eng::render::{ArcballCameraController, Camera, Renderer, Scene};
use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};

pub struct Content {
    renderer: Renderer,
    window: Window,
    scene: Scene,
    camera: Camera<ArcballCameraController>,
}

impl Content {
    pub async fn new(window: Window) -> Self {
        let package = WasmPackage::new(&regions()[0], "https://ffxiv-data.dlunch.net/compressed").await;

        let size = window.inner_size();
        let renderer = Renderer::new(&window, size.width, size.height).await;

        let controller = ArcballCameraController::new(Point3::new(0.0, 0.8, 0.0), 2.5);
        let camera = Camera::new(45.0 * PI / 180.0, size.width as f32 / size.height as f32, 0.1, 100.0, controller);
        let mut scene = Scene::new();
        let context = Context::new(&renderer, &package).await.unwrap();

        let mut equipments = HashMap::new();
        equipments.insert(ModelPart::Met, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Top, Equipment::new(6016, 1, 20));
        equipments.insert(ModelPart::Glv, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Dwn, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Sho, Equipment::new(6016, 1, 0));

        let customization = Customization::new(BodyId::AuRaFemale, 1, 1, 1, 1, 1);
        let character = Character::new(&renderer, &package, &context, customization, equipments).await.unwrap();

        scene.add(character);

        Self {
            renderer,
            window,
            scene,
            camera,
        }
    }

    pub fn redraw(&mut self) {
        self.renderer.render(&self.camera, &self.scene);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
