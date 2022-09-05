use core::f32::consts::PI;

use glam::Vec3;
use hashbrown::HashMap;
use winit::window::Window;

use common::{regions, WasmPackage};
use eng::{
    ecs::World,
    render::{ArcballCameraController, CameraComponent, PerspectiveCamera, Renderer},
};
use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};

pub struct Content {
    window: Window,
    world: World,
}

impl Content {
    pub async fn new(window: Window) -> Self {
        let mut world = World::new();

        let package = WasmPackage::new(&regions()[0], "https://ffxiv-data.dlunch.net/compressed").await;

        let size = window.inner_size();
        world.add_resource(Renderer::new(&window, size.width, size.height).await);

        let controller = ArcballCameraController::new(Vec3::new(0.0, 0.8, 0.0), 2.5);
        let camera = PerspectiveCamera::new(45.0 * PI / 180.0, 0.1, 100.0, controller);
        world.spawn().with(CameraComponent { camera: Box::new(camera) });

        let renderer = world.resource::<Renderer>().unwrap();
        let context = Context::new(&renderer, &package).await.unwrap();

        let mut equipments = HashMap::new();
        equipments.insert(ModelPart::Met, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Top, Equipment::new(6016, 1, 20));
        equipments.insert(ModelPart::Glv, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Dwn, Equipment::new(6016, 1, 0));
        equipments.insert(ModelPart::Sho, Equipment::new(6016, 1, 0));

        let customization = Customization::new(BodyId::AuRaFemale, 1, 1, 1, 1, 1);
        Character::load(&mut world, &package, &context, customization, equipments).await.unwrap();

        Self { window, world }
    }

    pub fn redraw(&mut self) {
        let mut renderer = self.world.take_resource::<Renderer>().unwrap();

        renderer.render_world(&self.world);

        self.world.add_resource(renderer);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
