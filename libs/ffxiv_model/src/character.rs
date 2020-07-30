use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use zerocopy::AsBytes;

use hashbrown::HashMap;

use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};

use renderer::{Buffer, RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result, SqPackReaderError};

use crate::{
    character_equipment_part::CharacterEquipmentPart, character_part::CharacterPart, constants::ModelPart, context::Context,
    customization::Customization, equipment::Equipment, model_reader::ModelReader,
};

pub struct Character {
    parts: Vec<Box<dyn Renderable>>,
    #[allow(dead_code)]
    bone_transform: Arc<Buffer>,
}

impl Character {
    pub async fn new(
        renderer: &Renderer,
        package: &dyn Package,
        context: &Context,
        customization: Customization,
        equipments: HashMap<ModelPart, Equipment>,
    ) -> Result<Self> {
        // TODO temp
        let bone_transforms = vec![1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.]
            .into_iter()
            .cycle()
            .take(4 * 3 * 64)
            .collect::<Vec<_>>();
        let bone_transform = Arc::new(renderer.buffer_pool.alloc(bone_transforms.len() * core::mem::size_of::<f32>()));
        bone_transform.write(bone_transforms.as_bytes()).await.unwrap();

        let read_futures = equipments
            .into_iter()
            .map(|(equipment_part, equipment)| ModelReader::read_equipment(renderer, package, &customization, equipment_part, equipment, context));
        let mut parts = read_futures
            .map(|x| {
                x.then(|data| async {
                    Ok::<Box<dyn Renderable>, SqPackReaderError>(Box::new(
                        CharacterEquipmentPart::new(renderer, data?.model_data, bone_transform.clone(), context).await,
                    ))
                })
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        // chaining part model futures and equipment read futures causes compiler issue https://github.com/rust-lang/rust/issues/64650
        let face_part_model = ModelReader::read_face(renderer, package, &customization, context).await?;
        let face_part = Box::new(CharacterPart::new(renderer, face_part_model, bone_transform.clone(), context).await);
        parts.push(face_part);

        let hair_part_model = ModelReader::read_hair(renderer, package, &customization, context).await?;
        let hair_part = Box::new(CharacterPart::new(renderer, hair_part_model, bone_transform.clone(), context).await);
        parts.push(hair_part);

        Ok(Self { parts, bone_transform })
    }
}

impl Renderable for Character {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(&mut render_context);
        }
    }
}
