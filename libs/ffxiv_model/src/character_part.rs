use std::collections::HashMap;
use std::sync::Arc;

use renderer::{Material, Mesh, MeshPart, Model, RenderContext, Renderable, Renderer, Texture, TextureFormat, VertexFormat, VertexFormatItem};

use crate::context::Context;
use crate::model_reader::ModelData;
use crate::type_adapter::{convert_buffer_type, convert_buffer_usage, convert_texture_name};

pub struct CharacterPart {
    models: Vec<Model>,
}

impl CharacterPart {
    pub async fn new(renderer: &Renderer, model_data: ModelData, context: &Context) -> Self {
        let mdl = model_data.mdl;

        let lod = 0;
        let meshes = mdl.meshes(lod);
        let buffer_items = mdl.buffer_items(lod);

        let mut models = Vec::new();
        for (mesh_index, (mesh_data, buffer_item)) in meshes.zip(buffer_items).enumerate() {
            let vertex_formats = (0..mesh_data.mesh_info.buffer_count as usize)
                .map(|buffer_index| {
                    let buffer_items = buffer_item.items().filter(move |x| x.buffer as usize == buffer_index);
                    VertexFormat::new(
                        buffer_items
                            .map(|x| VertexFormatItem::new(convert_buffer_usage(x.usage), convert_buffer_type(x.item_type), x.offset as usize))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            let strides = (0..mesh_data.mesh_info.buffer_count as usize)
                .map(|i| mesh_data.mesh_info.strides[i] as usize)
                .collect::<Vec<_>>();

            let mesh = Mesh::new(&renderer, mesh_data.buffers.as_ref(), &strides, mesh_data.indices, vertex_formats);

            let (mtrl, texs) = &model_data.mtrls[mesh_index];
            let mut textures = mtrl
                .parameters()
                .iter()
                .map(|parameter| {
                    (
                        convert_texture_name(parameter.parameter_type),
                        texs[parameter.texture_index as usize].clone(),
                    )
                })
                .collect::<HashMap<_, _>>();

            let color_table_data = mtrl.color_table();
            if !color_table_data.is_empty() {
                let color_table_tex = Texture::new(&renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float).await;
                textures.insert("ColorTable", Arc::new(color_table_tex));
            }

            let shaders = context.shader_holder.get_shaders(mtrl.shader_name());

            let material = Material::new(&renderer, textures, shaders.0, shaders.1);

            models.push(Model::new(
                &renderer,
                mesh,
                material,
                vec![MeshPart::new(0, mesh_data.mesh_info.index_count as u32)],
            ));
        }

        Self { models }
    }
}

impl Renderable for CharacterPart {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for model in &self.models {
            model.render(&mut render_context);
        }
    }
}
