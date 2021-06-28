use alloc::sync::Arc;

use hashbrown::HashMap;

use renderer::{Buffer, Material, Renderer, Texture};

use crate::{shader_holder::ShaderType, Context};

pub struct IrisMaterial {}

impl IrisMaterial {
    pub fn create(
        renderer: &Renderer,
        context: &Context,
        mut textures: HashMap<&'static str, Arc<Texture>>,
        uniforms: HashMap<&'static str, Arc<Buffer>>,
    ) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Iris);

        if !textures.contains_key("Diffuse") {
            textures.insert("Diffuse", context.empty_texture.clone());
        }

        Material::new(renderer, textures, uniforms, shader)
    }
}
