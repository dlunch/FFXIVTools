use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use eng::render::{Material, Renderer, Resource, Texture, TextureFormat};
use ffxiv_parser::{Mtrl, Stm};

use crate::{Context, shader_holder::ShaderType};

pub struct CharacterMaterial {}

impl CharacterMaterial {
    pub fn create<'a>(
        renderer: &'a Renderer,
        context: &'a Context,
        mtrl: &'a Mtrl,
        stain_id: u8,
        mut resources: HashMap<&'static str, Arc<dyn Resource>>,
    ) -> Material {
        let color_table_data = mtrl.color_table();
        if !color_table_data.is_empty() {
            let color_table_texels = Self::apply_staining(color_table_data, stain_id, &context.staining_template);
            let color_table_tex = Texture::with_texels(renderer, 4, 16, &color_table_texels, TextureFormat::Rgba16Float);
            resources.insert("color_table_tex", Arc::new(color_table_tex));
        } else {
            resources.insert("color_table_tex", context.empty_texture.clone());
        }

        if !resources.contains_key("mask_tex") {
            resources.insert("mask_tex", context.empty_texture.clone());
        }
        if !resources.contains_key("specular_tex") {
            resources.insert("specular_tex", context.empty_texture.clone());
        }

        let shader = context.shader_holder.shader(ShaderType::Character);

        Material::with_custom_shader(renderer, &resources.into_iter().collect::<Vec<_>>(), shader)
    }

    fn apply_staining(color_table_data: &[u8], stain_id: u8, staining_template: &Stm) -> Vec<u8> {
        if color_table_data.len() == 4 * 16 * 8 || stain_id == 0 {
            color_table_data[..4 * 16 * 8].to_vec()
        } else {
            let mut result = color_table_data[..4 * 16 * 8].to_vec();

            for i in 0..16 {
                let stain_data = Self::u8_to_u16(&color_table_data[(4 * 16 * 4 + i) * 2..]);
                if stain_data & 0x1f != 0 {
                    let template_data = staining_template.get(stain_data >> 5);

                    let row = &mut result[(i * 16) * 2..];
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

        let begin = 2 * (last + count * data_offset);
        let end = begin + count * 2;
        row.copy_from_slice(&table_data[begin..end]);
    }

    fn u8_to_u16(u8: &[u8]) -> u16 {
        u16::from_le_bytes([u8[0], u8[1]])
    }
}
