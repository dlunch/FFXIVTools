use renderer::{RenderContext, Renderable, Renderer};
use sqpack_reader::{Package, Result};

use crate::character_part::CharacterPart;
use crate::model_read_context::ModelReadContext;
use crate::shader_holder::ShaderHolder;

pub struct Character<'a> {
    renderer: &'a Renderer,
    package: &'a dyn Package,
    shader_holder: &'a ShaderHolder,
    parts: Vec<CharacterPart<'a>>,
}

impl<'a> Character<'a> {
    pub fn new(renderer: &'a Renderer, package: &'a dyn Package, shader_holder: &'a ShaderHolder) -> Self {
        Self {
            renderer,
            package,
            shader_holder,
            parts: Vec::new(),
        }
    }

    pub async fn add_equipment(&'a mut self) -> Result<()> {
        // WIP
        let read_context = ModelReadContext::read_equipment(self.package, 6016, 201, "top").await?;
        let part = CharacterPart::new(self.renderer, read_context, &self.shader_holder).await;
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
