use alloc::sync::Arc;
use core::future::Future;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Buffer, Material, Renderer, Texture, TextureFormat};

use crate::{shader_holder::ShaderType, Context};

pub struct CharacterMaterial {}

impl CharacterMaterial {
    pub fn create<'a>(
        renderer: &'a Renderer,
        context: &'a Context,
        mtrl: &'a Mtrl,
        mut textures: HashMap<&'static str, Arc<Texture>>,
        uniforms: HashMap<&'static str, Arc<Buffer>>,
    ) -> impl Future<Output = Material> + 'a {
        // NOTE return async closure to workaround https://github.com/rust-lang/rust/issues/63033
        async move {
            let color_table_data = mtrl.color_table();
            if !color_table_data.is_empty() {
                let color_table_texels = &color_table_data[..4 * 16 * 8];
                let color_table_tex = Texture::with_texels(&renderer, 4, 16, color_table_texels, TextureFormat::Rgba16Float).await;
                textures.insert("ColorTable", Arc::new(color_table_tex));
            } else {
                textures.insert("ColorTable", context.empty_texture.clone());
            }

            if !textures.contains_key("Mask") {
                textures.insert("Mask", context.empty_texture.clone());
            }

            let vertex_shader = context.shader_holder.vertex_shader.clone();
            let fragment_shader = context.shader_holder.fragment_shader(ShaderType::Character);

            Material::new(&renderer, textures, uniforms, vertex_shader, fragment_shader)
        }
    }
}
