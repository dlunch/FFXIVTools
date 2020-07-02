use alloc::sync::Arc;

use hashbrown::HashMap;

use renderer::{Buffer, Material, Renderer, Texture};

use crate::{shader_holder::ShaderType, Context};

pub struct HairMaterial {}

impl HairMaterial {
    pub fn create(
        renderer: &Renderer,
        context: &Context,
        textures: &mut HashMap<&'static str, Arc<Texture>>,
        uniforms: &HashMap<&'static str, Arc<Buffer>>,
    ) -> Material {
        let vertex_shader = context.shader_holder.vertex_shader.clone();
        let fragment_shader = context.shader_holder.fragment_shader(ShaderType::Hair);

        if !textures.contains_key("Diffuse") {
            textures.insert("Diffuse", context.empty_texture.clone());
        }

        Material::new(&renderer, textures, uniforms, vertex_shader, fragment_shader)
    }
}
