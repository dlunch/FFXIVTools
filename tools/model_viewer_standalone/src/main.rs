use std::f32::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use glam::Vec3;
use hashbrown::HashMap;
use log::debug;
use tokio::{fs, task, time};

use eng::{
    App,
    ecs::World,
    render::{ArcballCameraController, CameraComponent, PerspectiveCamera, Renderer},
};
use ffxiv_model::{BodyId, Character, Context, Customization, Equipment, ModelPart};
use sqpack::SqPackPackage;
use sqpack_extension::{BatchedPackage, ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    App::new().await.setup(setup).await.run()
}

async fn setup(world: &mut World) {
    #[cfg(unix)]
    let path = "/mnt/d/Games/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack";
    #[cfg(windows)]
    let path = "D:\\Games\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack";

    let package = if fs::metadata(path).await.is_ok() {
        BatchedPackage::new(SqPackPackage::new(Path::new(path)).unwrap())
    } else {
        let provider = ExtractedFileProviderWeb::with_progress("https://ffxiv-data.dlunch.net/compressed/all/", |current, total| {
            debug!("{current}/{total}")
        });
        BatchedPackage::new(SqPackReaderExtractedFile::new(provider))
    };
    let package = Arc::new(package);

    let package2 = package.clone();
    task::spawn(async move {
        loop {
            package2.poll().await.unwrap();
            time::sleep(Duration::from_millis(16)).await;
        }
    });

    let renderer = world.resource::<Renderer>().unwrap();
    let context = Context::new(renderer, &*package).await.unwrap();

    let controller = ArcballCameraController::new(Vec3::new(0.0, 0.8, 0.0), 2.5);
    let camera = PerspectiveCamera::new(45.0 * PI / 180.0, 0.1, 100.0, controller);
    world.spawn().with(CameraComponent { camera: Box::new(camera) });

    let mut equipments = HashMap::new();
    equipments.insert(ModelPart::Met, Equipment::new(6016, 1, 0));
    equipments.insert(ModelPart::Top, Equipment::new(6016, 1, 20));
    equipments.insert(ModelPart::Glv, Equipment::new(6016, 1, 0));
    equipments.insert(ModelPart::Dwn, Equipment::new(6016, 1, 0));
    equipments.insert(ModelPart::Sho, Equipment::new(6016, 1, 0));

    let customization = Customization::new(BodyId::AuRaFemale, 1, 1, 1, 1, 1);
    Character::load(world, &*package, &context, customization, equipments).await.unwrap();
}
