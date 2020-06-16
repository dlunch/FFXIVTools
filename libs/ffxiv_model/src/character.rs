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

pub struct Character<'a> {
    renderer: &'a Renderer,
    package: &'a dyn Package,
    context: &'a Context,
    customization: Customization,
    parts: Vec<CharacterPart>,
}

impl<'a> Character<'a> {
    pub async fn new(
        renderer: &'a Renderer,
        package: &'a dyn Package,
        context: &'a Context,
        customization: Customization,
        equipments: HashMap<ModelPart, Equipment>,
    ) -> Result<Character<'a>> {
        let mut result = Self {
            renderer,
            package,
            context,
            customization,
            parts: Vec::new(),
        };

        let read_futures = equipments.into_iter().map(|(equipment_part, equipment)| {
            ModelReader::read_equipment(
                result.renderer,
                result.package,
                &result.customization,
                equipment_part,
                equipment,
                result.context,
            )
        });
        result.parts = read_futures
            .map(|x| x.then(|data| async { Ok::<_, SqPackReaderError>(CharacterPart::new(result.renderer, data?, result.context).await) }))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        // chaining part model futures and equipment read futures causes compiler issue https://github.com/rust-lang/rust/issues/64650
        let face_part_model = ModelReader::read_face(result.renderer, result.package, &result.customization, result.context).await?;
        let face_part = CharacterPart::new(result.renderer, face_part_model, result.context).await;
        result.parts.push(face_part);

        let hair_part_model = ModelReader::read_hair(result.renderer, result.package, &result.customization, result.context).await?;
        let hair_part = CharacterPart::new(result.renderer, hair_part_model, result.context).await;
        result.parts.push(hair_part);

        Ok(result)
    }
}

impl Renderable for Character<'_> {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(&mut render_context);
        }
    }
}
