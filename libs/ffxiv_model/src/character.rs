use std::collections::HashMap;

use futures::future;
use futures::FutureExt;

use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result};

use crate::character_part::CharacterPart;
use crate::constants::{BodyId, ModelPart};
use crate::model_reader::ModelReader;
use crate::shader_holder::ShaderHolder;

pub struct Character<'a> {
    renderer: &'a Renderer,
    package: &'a dyn Package,
    shader_holder: &'a ShaderHolder,
    parts: Vec<CharacterPart>,
    body_id: BodyId,
    body_type: u16,
    body_variant_id: u16,
}

impl<'a> Character<'a> {
    pub async fn new(
        renderer: &'a Renderer,
        package: &'a dyn Package,
        shader_holder: &'a ShaderHolder,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipments: HashMap<ModelPart, (u16, u16)>,
    ) -> Result<Character<'a>> {
        let mut result = Self {
            renderer,
            package,
            shader_holder,
            parts: Vec::new(),
            body_id,
            body_type,
            body_variant_id,
        };

        result.parts = future::join_all(equipments.into_iter().map(|(equipment_part, (equipment_id, equipment_variant_id))| {
            ModelReader::read_equipment(
                result.package,
                result.body_id,
                result.body_type,
                result.body_variant_id,
                equipment_id,
                equipment_variant_id,
                equipment_part,
            )
            .then(|data| async { Ok(CharacterPart::new(result.renderer, data?, result.shader_holder).await) })
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

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
