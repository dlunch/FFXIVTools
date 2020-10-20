use alloc::{sync::Arc, vec::Vec};
use core::future::Future;

use hashbrown::HashMap;

use ffxiv_parser::{Mtrl, Stm};
use renderer::{Buffer, Material, Renderer, Texture, TextureFormat};

use crate::{shader_holder::ShaderType, Context};

pub struct CharacterMaterial {}

impl CharacterMaterial {
    #[allow(clippy::manual_async_fn)]
    pub fn create<'a>(
        renderer: &'a Renderer,
        context: &'a Context,
        mtrl: &'a Mtrl,
        stain_id: u8,
        mut textures: HashMap<&'static str, Arc<Texture>>,
        uniforms: HashMap<&'static str, Arc<Buffer>>,
    ) -> impl Future<Output = Material> + 'a {
        // NOTE return async closure to workaround https://github.com/rust-lang/rust/issues/63033
        async move {
            let color_table_data = mtrl.color_table();
            if !color_table_data.is_empty() {
                let color_table_texels = Self::apply_staining(color_table_data, stain_id, &context.staining_template);
                let color_table_tex = Texture::with_texels(&renderer, 4, 16, &color_table_texels, TextureFormat::Rgba16Float).await;
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

    fn apply_staining(color_table_data: &[u8], stain_id: u8, staining_template: &Stm) -> Vec<u8> {
        if color_table_data.len() == 4 * 16 * 8 || stain_id == 0 {
            color_table_data.to_vec()
        } else {
            let mut result = color_table_data[..4 * 16 * 8].to_vec();

            for i in 0..16 {
                let stain_data = Self::u8_to_u16(&color_table_data[(4 * 16 * 4 + i) * 2..]);
                if stain_data & 0x1f != 0 {
                    let template_data = staining_template.get(stain_data >> 5);

                    let row = &mut result[i * 16..];
                    if stain_data & 1 != 0 {
                        Self::apply_staining_row(template_data, stain_id, 0, &mut row[..6], 3);
                    }
                    if stain_data & 2 != 0 {
                        Self::apply_staining_row(template_data, stain_id, 1, &mut row[8..14], 3);
                    }
                    if stain_data & 4 != 0 {
                        Self::apply_staining_row(template_data, stain_id, 2, &mut row[16..22], 3);
                    }
                    if stain_data & 8 != 0 {
                        Self::apply_staining_row(template_data, stain_id, 3, &mut row[14..16], 1);
                    }
                    if stain_data & 16 != 0 {
                        Self::apply_staining_row(template_data, stain_id, 4, &mut row[6..8], 1);
                    }
                }
            }

            result
        }
    }

    fn apply_staining_row(template_data: &[u8], stain_id: u8, stain_type: usize, row: &mut [u8], count: usize) {
        let table_data = &template_data[10..];

        let last = if stain_type > 0 {
            Self::u8_to_u16(&template_data[(stain_type - 1) * 2..]) as usize
        } else {
            0
        };
        let current = Self::u8_to_u16(&template_data[stain_type * 2..]) as usize;
        let offset = (current - last) / count;
        let data_offset = if offset != 1 && offset != 128 {
            table_data[stain_id as usize + current * 2 - 128] as usize - 1
        } else {
            stain_id as usize - 1
        };

        let begin = 2 * (last + count * data_offset) as usize;
        let end = begin + count * 2;
        row.copy_from_slice(&table_data[begin..end]);
    }

    fn u8_to_u16(u8: &[u8]) -> u16 {
        u16::from_le_bytes([u8[0], u8[1]])
    }
}
