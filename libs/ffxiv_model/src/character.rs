use alloc::{boxed::Box, vec::Vec};

use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};
use hashbrown::HashMap;

use eng::render::{RenderContext, Renderable, Renderer};
use sqpack::{Package, Result, SqPackReaderError};

use crate::{
    character_part::CharacterPart, constants::ModelPart, context::Context, customization::Customization, equipment::Equipment,
    model_reader::ModelReader,
};

pub struct Character {
    parts: Vec<Box<dyn Renderable>>,
}

impl Character {
    pub async fn new(
        renderer: &Renderer,
        package: &dyn Package,
        context: &Context,
        customization: Customization,
        equipments: HashMap<ModelPart, Equipment>,
    ) -> Result<Self> {
        let bone_transforms = HashMap::new();

        let read_futures = equipments
            .into_iter()
            .map(|(equipment_part, equipment)| ModelReader::read_equipment(renderer, package, &customization, equipment_part, equipment, context));
        let parts_fut = read_futures
            .map(|x| {
                x.map(|data| {
                    Ok::<Box<dyn Renderable>, SqPackReaderError>(Box::new(CharacterPart::with_equipment_model(
                        renderer,
                        data?,
                        &bone_transforms,
                        context,
                        &customization,
                    )))
                })
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>();

        // chaining part model futures and equipment read futures requires boxed future, emits strange compile error https://github.com/rust-lang/rust/issues/64650
        let face_part_fut = ModelReader::read_face(renderer, package, &customization, context).map(|x| {
            Ok::<Box<dyn Renderable>, SqPackReaderError>(Box::new(CharacterPart::with_model(
                renderer,
                x?,
                &bone_transforms,
                context,
                &customization,
            )))
        });

        let hair_part_fut = ModelReader::read_hair(renderer, package, &customization, context).map(|x| {
            Ok::<Box<dyn Renderable>, SqPackReaderError>(Box::new(CharacterPart::with_model(
                renderer,
                x?,
                &bone_transforms,
                context,
                &customization,
            )))
        });

        let (mut parts, face_part, hair_part) = futures::future::try_join3(parts_fut, face_part_fut, hair_part_fut).await?;

        parts.push(face_part);
        parts.push(hair_part);

        Ok(Self { parts })
    }
}

impl Renderable for Character {
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(render_context);
        }
    }
}
