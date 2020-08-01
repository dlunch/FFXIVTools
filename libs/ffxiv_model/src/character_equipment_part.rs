use alloc::string::String;

use hashbrown::HashMap;
use nalgebra::Matrix4;

use renderer::{RenderContext, Renderable, Renderer};

use crate::{character_part::CharacterPart, context::Context, model_reader::EquipmentModelData};

pub struct CharacterEquipmentPart {
    part: CharacterPart,
}

impl CharacterEquipmentPart {
    pub async fn new(
        renderer: &Renderer,
        model_data: EquipmentModelData,
        _bone_transforms: &HashMap<String, Matrix4<f32>>,
        context: &Context,
    ) -> Self {
        log::debug!(
            "original {:?} deformed {:?}",
            model_data.original_body_id as u16,
            model_data.deformed_body_id as u16
        );
        let prebone_deformer = context.get_body_deform_matrices(model_data.original_body_id, model_data.deformed_body_id);
        let part = CharacterPart::new(renderer, model_data.model_data, &prebone_deformer, context).await;

        Self { part }
    }
}

impl Renderable for CharacterEquipmentPart {
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>) {
        self.part.render(render_context)
    }
}
