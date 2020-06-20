use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture, TextureFormat};

use crate::{shader_holder::ShaderType, Context};

pub struct CharacterMaterial {}

impl CharacterMaterial {
    pub async fn create(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &mut HashMap<&'static str, Arc<Texture>>) -> Material {
        let color_table_data = mtrl.color_table();
        if !color_table_data.is_empty() {
            let color_table_tex = Texture::with_texels(&renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float).await;
            textures.insert("ColorTable", Arc::new(color_table_tex));
        } else {
            textures.insert("ColorTable", context.empty_texture.clone());
        }

        if !textures.contains_key("Mask") {
            textures.insert("Mask", context.empty_texture.clone());
        }

        let vertex_shader = context.shader_holder.vertex_shader.clone();
        let fragment_shader = context.shader_holder.fragment_shader(ShaderType::Hair);

        Material::new(&renderer, &textures, vertex_shader, fragment_shader)
    }
}
