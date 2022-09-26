use alloc::vec::Vec;

use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};
use hashbrown::HashMap;

use eng::{
    ecs::{HierarchyExt, World},
    render::{RenderBundle, Renderer},
};
use sqpack::{Package, Result, SqPackReaderError};

use crate::{
    character_part::CharacterPart, constants::ModelPart, context::Context, customization::Customization, equipment::Equipment,
    model_reader::ModelReader,
};

pub struct Character {}

impl Character {
    pub async fn load(
        world: &mut World,
        package: &dyn Package,
        context: &Context,
        customization: Customization,
        equipments: HashMap<ModelPart, Equipment>,
    ) -> Result<()> {
        let entity = world.spawn().entity();

        let renderer = world.resource::<Renderer>().unwrap();

        let bone_transforms = HashMap::new();

        let read_futures = equipments
            .into_iter()
            .map(|(equipment_part, equipment)| ModelReader::read_equipment(renderer, package, &customization, equipment_part, equipment, context));
        let parts_fut = read_futures
            .map(|x| {
                x.map(|data| {
                    Ok::<Vec<RenderBundle>, SqPackReaderError>(CharacterPart::load_equipment_model(
                        renderer,
                        data?,
                        &bone_transforms,
                        context,
                        &customization,
                    ))
                })
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>();

        // chaining part model futures and equipment read futures requires boxed future, emits strange compile error https://github.com/rust-lang/rust/issues/64650
        let face_part_fut = ModelReader::read_face(renderer, package, &customization, context)
            .map(|x| Ok::<Vec<RenderBundle>, SqPackReaderError>(CharacterPart::load_model(renderer, x?, &bone_transforms, context, &customization)));

        let hair_part_fut = ModelReader::read_hair(renderer, package, &customization, context)
            .map(|x| Ok::<Vec<RenderBundle>, SqPackReaderError>(CharacterPart::load_model(renderer, x?, &bone_transforms, context, &customization)));

        let (parts, face_part, hair_part) = futures::future::try_join3(parts_fut, face_part_fut, hair_part_fut).await?;

        for part in parts {
            let part_entity = world.spawn().entity();
            world.add_child(entity, part_entity);

            for bundle in part {
                let part_child_entity = world.spawn_bundle(bundle);
                world.add_child(part_entity, part_child_entity);
            }
        }

        for bundle in face_part.into_iter().chain(hair_part.into_iter()) {
            let part_entity = world.spawn_bundle(bundle);
            world.add_child(entity, part_entity);
        }

        Ok(())
    }
}
