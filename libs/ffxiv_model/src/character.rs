use std::collections::HashMap;

use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};

use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result, SqPackReaderError};

use crate::character_part::CharacterPart;
use crate::constants::{BodyId, ModelPart};
use crate::context::Context;
use crate::model_reader::ModelReader;

pub struct Character<'a> {
    renderer: &'a Renderer,
    package: &'a dyn Package,
    context: &'a Context,
    parts: Vec<CharacterPart>,
    body_id: BodyId,
    body_type: u16,
    body_variant_id: u16,
}

impl<'a> Character<'a> {
    pub async fn new(
        renderer: &'a Renderer,
        package: &'a dyn Package,
        context: &'a Context,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipments: HashMap<ModelPart, (u16, u16)>,
    ) -> Result<Character<'a>> {
        let mut result = Self {
            renderer,
            package,
            context,
            parts: Vec::new(),
            body_id,
            body_type,
            body_variant_id,
        };

        let read_futures = equipments.into_iter().map(|(equipment_part, (equipment_id, equipment_variant_id))| {
            ModelReader::read_equipment(
                result.renderer,
                result.package,
                result.body_id,
                result.body_type,
                result.body_variant_id,
                equipment_id,
                equipment_variant_id,
                equipment_part,
                result.context,
            )
        });
        result.parts = read_futures
            .map(|x| x.then(|data| async { Ok::<_, SqPackReaderError>(CharacterPart::new(result.renderer, data?, result.context).await) }))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        // chaining part model futures and equipment read futures causes compiler issue https://github.com/rust-lang/rust/issues/64650
        let face_part_model = ModelReader::read_face(result.renderer, result.package, result.body_id, 1, result.context).await?;
        let face_part = CharacterPart::new(result.renderer, face_part_model, result.context).await;
        result.parts.push(face_part);

        let hair_part_model = ModelReader::read_hair(result.renderer, result.package, result.body_id, 1, 1, result.context).await?;
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
