use std::sync::Arc;

use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result};

use crate::character_part::CharacterPart;
use crate::model_read_context::ModelReadContext;
use crate::shader_holder::ShaderHolder;

pub struct Character {
    parts: Vec<CharacterPart>,
    shader_holder: Arc<ShaderHolder>,
}

impl Character {
    pub fn new(shader_holder: Arc<ShaderHolder>) -> Self {
        Self {
            parts: Vec::new(),
            shader_holder,
        }
    }

    pub async fn add_equipment(&mut self, renderer: &Renderer, package: &dyn Package) -> Result<()> {
        // WIP
        let read_context = ModelReadContext::read_equipment(package, 6016, 201, "top").await?;
        let part = CharacterPart::new(renderer, read_context, &self.shader_holder).await;
        self.parts.push(part);

        Ok(())
    }
}

impl Renderable for Character {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for part in &self.parts {
            part.render(&mut render_context);
        }
    }
}
