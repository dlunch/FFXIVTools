use alloc::sync::Arc;

use renderer::{Buffer, RenderContext, Renderable, Renderer};

use crate::{character_part::CharacterPart, context::Context, model_reader::EquipmentModelData};

pub struct CharacterEquipmentPart {
    part: CharacterPart,
}

impl CharacterEquipmentPart {
    pub async fn new(renderer: &Renderer, model_data: EquipmentModelData, bone_transform: Arc<Buffer>, context: &Context) -> Self {
        let part = CharacterPart::new(renderer, model_data.model_data, bone_transform, context).await;

        Self { part }
    }
}

impl Renderable for CharacterEquipmentPart {
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>) {
        self.part.render(render_context)
    }
}
