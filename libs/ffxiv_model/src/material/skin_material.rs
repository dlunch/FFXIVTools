use alloc::sync::Arc;

use hashbrown::HashMap;

use renderer::{Buffer, Material, Renderer, Texture};

use crate::{shader_holder::ShaderType, Context};

pub struct SkinMaterial {}

impl SkinMaterial {
    pub fn create(
        renderer: &Renderer,
        context: &Context,
        textures: HashMap<&'static str, Arc<Texture>>,
        uniforms: HashMap<&'static str, Arc<Buffer>>,
    ) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Skin);

        Material::new(&renderer, textures, uniforms, shader)
    }
}
