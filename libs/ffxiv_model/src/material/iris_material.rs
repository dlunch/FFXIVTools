use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use eng::render::{Material, Renderer, Resource};

use crate::{Context, shader_holder::ShaderType};

pub struct IrisMaterial {}

impl IrisMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mut resources: HashMap<&'static str, Arc<dyn Resource>>) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Iris);

        if !resources.contains_key("diffuse_tex") {
            resources.insert("diffuse_tex", context.empty_texture.clone());
        }

        Material::with_custom_shader(renderer, &resources.into_iter().collect::<Vec<_>>(), shader)
    }
}
