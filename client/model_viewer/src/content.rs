use hashbrown::HashMap;
use nalgebra::Point3;
use winit::window::Window;

use common::{regions, WasmPackage};
use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};
use renderer::{Camera, Renderer, Scene};

pub struct Content {
    renderer: Renderer,
    window: Window,
    scene: Scene,
}

impl Content {
    pub async fn new(window: Window) -> Self {
        let package = WasmPackage::new(&regions()[0], "https://ffxiv-data.dlunch.net/compressed").await;

        let size = window.inner_size();
        let renderer = Renderer::new(&window, size.width, size.height).await;

        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        let mut scene = Scene::new(camera);
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

        Self { renderer, window, scene }
    }

    pub fn redraw(&mut self) {
        self.renderer.render(&self.scene);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
