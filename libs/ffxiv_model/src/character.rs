use std::collections::HashMap;
use std::sync::Arc;

use futures::{future, FutureExt};

use renderer::{Material, Mesh, Model, RenderContext, Renderable, Renderer, Texture, TextureFormat, VertexFormat, VertexFormatItem};
use sqpack_reader::{Package, Result};

use crate::model_read_context::ModelReadContext;
use crate::shader_holder::ShaderHolder;
use crate::type_adapter::{convert_buffer_type, convert_buffer_usage, convert_texture_name, load_texture};

pub struct Character {
    models: Vec<Model>,
    shader_holder: Arc<ShaderHolder>,
}

impl Character {
    pub fn new(shader_holder: Arc<ShaderHolder>) -> Self {
        Self {
            models: Vec::new(),
            shader_holder,
        }
    }

    pub async fn add_equipment(&mut self, renderer: &Renderer, pack: &dyn Package) -> Result<()> {
        // WIP
        let read_context = ModelReadContext::read_equipment(pack, 6016, 201, "top").await?;
        let mdl = read_context.mdl;

        let quality = 0;
        let meshes = mdl.meshes(quality);
        let buffer_items = mdl.buffer_items(quality);

        for (mesh_index, (mesh, buffer_item)) in meshes.zip(buffer_items).enumerate() {
            let vertex_formats = (0..mesh.mesh_info.buffer_count as usize)
                .map(|buffer_index| {
                    let buffer_items = buffer_item.items().filter(move |x| x.buffer as usize == buffer_index);
                    VertexFormat::new(
                        buffer_items
                            .map(|x| VertexFormatItem::new(convert_buffer_usage(x.usage), convert_buffer_type(x.item_type), x.offset as usize))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            let strides = (0..mesh.mesh_info.buffer_count as usize)
                .map(|i| mesh.mesh_info.strides[i] as usize)
                .collect::<Vec<_>>();

            let mesh = Mesh::new(
                &renderer,
                mesh.buffers.as_ref(),
                &strides,
                mesh.indices,
                mesh.mesh_info.index_count as usize,
                vertex_formats,
            );

            let (mtrl, texs) = &read_context.mtrls[mesh_index];
            let mut textures = future::join_all(mtrl.parameters().iter().map(|parameter| {
                let tex_name = convert_texture_name(parameter.parameter_type);
                load_texture(&renderer, &texs[parameter.texture_index as usize]).map(move |x| (tex_name, x))
            }))
            .await
            .into_iter()
            .collect::<HashMap<_, _>>();

            let color_table_data = mtrl.color_table();
            if !color_table_data.is_empty() {
                let color_table_tex = Texture::new(&renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float).await;
                textures.insert("ColorTable", color_table_tex);
            }

            let shaders = self.shader_holder.get_shaders(mtrl.shader_name());

            let material = Material::new(&renderer, textures, shaders.0, shaders.1);

            self.models.push(Model::new(&renderer, mesh, material));
        }

        Ok(())
    }
}

impl Renderable for Character {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for model in &self.models {
            model.render(&mut render_context);
        }
    }
}
