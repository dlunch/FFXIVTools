use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub struct IrisMaterial {}

impl IrisMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &mut HashMap<&'static str, Arc<Texture>>) -> Material {
        let shaders = context.shader_holder.get_shaders(mtrl.shader_name());

        if !textures.contains_key("Diffuse") {
            textures.insert("Diffuse", context.empty_texture.clone());
        }

        Material::new(&renderer, textures, shaders.0, shaders.1)
    }
}
