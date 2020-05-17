use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result};

use crate::character_part::CharacterPart;
use crate::model_read_context::ModelReadContext;
use crate::shader_holder::ShaderHolder;
use crate::ModelPart;

pub struct Character<'a> {
    renderer: &'a Renderer,
    package: &'a dyn Package,
    shader_holder: &'a ShaderHolder,
    parts: Vec<CharacterPart>,
    body_id: u16,
    body_type: u16,
    body_variant_id: u16,
}

impl<'a> Character<'a> {
    pub fn new(
        renderer: &'a Renderer,
        package: &'a dyn Package,
        shader_holder: &'a ShaderHolder,
        body_id: u16,
        body_type: u16,
        body_variant_id: u16,
    ) -> Self {
        Self {
            renderer,
            package,
            shader_holder,
            parts: Vec::new(),
            body_id,
            body_type,
            body_variant_id,
        }
    }

    pub async fn add_equipment(&mut self, equipment_id: u16, equipment_variant_id: u16, equipment_part: ModelPart) -> Result<()> {
        let read_context = ModelReadContext::read_equipment(
            self.package,
            self.body_id,
            self.body_type,
            self.body_variant_id,
            equipment_id,
            equipment_variant_id,
            equipment_part,
        )
        .await?;
        let part = CharacterPart::new(self.renderer, read_context, self.shader_holder).await;
        self.parts.push(part);

        Ok(())
    }
}

impl Renderable for Character<'_> {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(&mut render_context);
        }
    }
}
