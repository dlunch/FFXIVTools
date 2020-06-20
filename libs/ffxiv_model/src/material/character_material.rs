use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture, TextureFormat};

use crate::Context;

pub struct CharacterMaterial {}

impl CharacterMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mtrl: &Mtrl, mut textures: HashMap<&'static str, Arc<Texture>>) -> Material {
        let color_table_data = mtrl.color_table();
        if !color_table_data.is_empty() {
            let color_table_tex = Texture::new(&renderer, 4, 16, Some(color_table_data), TextureFormat::Rgba16Float);
            textures.insert("ColorTable", Arc::new(color_table_tex));
        } else {
            textures.insert("ColorTable", context.empty_texture.clone());
        }

        if !textures.contains_key("Mask") {
            textures.insert("Mask", context.empty_texture.clone());
        }

        let shaders = context.shader_holder.get_shaders(mtrl.shader_name());

        Material::new(&renderer, textures, shaders.0, shaders.1)
    }
}
