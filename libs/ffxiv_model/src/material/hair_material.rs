use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use eng::render::{Material, Renderer, Resource};

use crate::{shader_holder::ShaderType, Context};

pub struct HairMaterial {}

impl HairMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mut resources: HashMap<&'static str, Arc<dyn Resource>>) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Hair);

        if !resources.contains_key("diffuse_tex") {
            resources.insert("diffuse_tex", context.empty_texture.clone());
        }

        Material::with_custom_shader(renderer, &resources.into_iter().collect::<Vec<_>>(), shader)
    }
}
