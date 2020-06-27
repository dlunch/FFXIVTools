use alloc::sync::Arc;

use hashbrown::HashMap;

use renderer::{Material, Renderer, Texture};

use crate::{shader_holder::ShaderType, Context};

pub struct SkinMaterial {}

impl SkinMaterial {
    pub fn create(renderer: &Renderer, context: &Context, textures: &mut HashMap<&'static str, Arc<Texture>>) -> Material {
        let vertex_shader = context.shader_holder.vertex_shader.clone();
        let fragment_shader = context.shader_holder.fragment_shader(ShaderType::Skin);

        Material::new(&renderer, textures, &HashMap::new(), vertex_shader, fragment_shader)
    }
}
