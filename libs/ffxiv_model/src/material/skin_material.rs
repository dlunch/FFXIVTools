use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub struct SkinMaterial {}

impl SkinMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: HashMap<&'static str, Arc<Texture>>) -> Material {
        let shaders = context.shader_holder.get_shaders(mtrl.shader_name());

        Material::new(&renderer, textures, shaders.0, shaders.1)
    }
}
