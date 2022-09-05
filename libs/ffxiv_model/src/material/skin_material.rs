use alloc::{sync::Arc, vec::Vec};

use eng::render::{Material, Renderer, Resource};
use hashbrown::HashMap;

use crate::{shader_holder::ShaderType, Context};

pub struct SkinMaterial {}

impl SkinMaterial {
    pub fn create(renderer: &Renderer, context: &Context, resources: HashMap<&'static str, Arc<dyn Resource>>) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Skin);

        let resources = resources.into_iter().collect::<Vec<_>>();

        Material::with_custom_shader(renderer, &resources, shader)
    }
}
