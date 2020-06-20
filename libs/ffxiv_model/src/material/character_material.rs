use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture, TextureFormat};

use crate::Context;

pub struct CharacterMaterial {}

impl CharacterMaterial {
    pub fn create(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &Vec<Arc<Texture>>) -> Material {
        let mut textures = mtrl
            .parameters()
            .iter()
            .map(|parameter| (parameter.parameter_type.as_str(), textures[parameter.texture_index as usize].clone()))
            .collect::<HashMap<_, _>>();

        let color_table_data = mtrl.color_table();
        if !color_table_data.is_empty() {
            let color_table_tex = Texture::new(&renderer, 4, 16, Some(color_table_data), TextureFormat::Rgba16Float);
            textures.insert("ColorTable", Arc::new(color_table_tex));
        } else {
            textures.insert("ColorTable", context.empty_texture.clone());
        }

        let shaders = context.shader_holder.get_shaders(mtrl.shader_name());

        Material::new(&renderer, textures, shaders.0, shaders.1)
    }
}
