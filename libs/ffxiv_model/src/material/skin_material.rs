use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use eng::render::{Buffer, Material, Renderer, Texture};

use crate::{shader_holder::ShaderType, Context};

pub struct SkinMaterial {}

impl SkinMaterial {
    pub fn create(
        renderer: &Renderer,
        context: &Context,
        textures: HashMap<&'static str, Arc<Texture>>,
        uniforms: &[(&'static str, Arc<Buffer>)],
    ) -> Material {
        let shader = context.shader_holder.shader(ShaderType::Skin);

        Material::new(renderer, &textures.into_iter().collect::<Vec<_>>(), uniforms, shader)
    }
}
