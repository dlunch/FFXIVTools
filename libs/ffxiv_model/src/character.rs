use alloc::vec::Vec;

use hashbrown::HashMap;

use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};

use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result, SqPackReaderError};

use crate::{
    character_part::CharacterPart, constants::ModelPart, context::Context, customization::Customization, equipment::Equipment,
    model_reader::ModelReader,
};

pub struct Character {
    parts: Vec<CharacterPart>,
}

impl Character {
    pub async fn new(
        renderer: &Renderer,
        package: &dyn Package,
        context: &Context,
        customization: Customization,
        equipments: HashMap<ModelPart, Equipment>,
    ) -> Result<Self> {
        let read_futures = equipments
            .into_iter()
            .map(|(equipment_part, equipment)| ModelReader::read_equipment(renderer, package, &customization, equipment_part, equipment, context));
        let mut parts = read_futures
            .map(|x| x.then(|data| async { Ok::<_, SqPackReaderError>(CharacterPart::new(renderer, data?, context).await) }))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        // chaining part model futures and equipment read futures causes compiler issue https://github.com/rust-lang/rust/issues/64650
        let face_part_model = ModelReader::read_face(renderer, package, &customization, context).await?;
        let face_part = CharacterPart::new(renderer, face_part_model, context).await;
        parts.push(face_part);

        let hair_part_model = ModelReader::read_hair(renderer, package, &customization, context).await?;
        let hair_part = CharacterPart::new(renderer, hair_part_model, context).await;
        parts.push(hair_part);

        Ok(Self { parts })
    }
}

impl Renderable for Character {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(&mut render_context);
        }
    }
}
