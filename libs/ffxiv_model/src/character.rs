use std::collections::HashMap;

use futures::{future, FutureExt};
use maplit::hashmap;

use renderer::{
    Material, Mesh, Model, RenderContext, Renderable, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture, TextureFormat, VertexFormat,
    VertexFormatItem,
};
use sqpack_reader::{Package, Result};

use crate::model_read_context::ModelReadContext;
use crate::type_adapter::{convert_buffer_type, convert_buffer_usage, convert_texture_name, load_texture};

pub struct Character {
    models: Vec<Model>,
}

impl Character {
    pub async fn new(pack: &dyn Package, renderer: &Renderer) -> Result<Self> {
        // WIP
        let read_context = ModelReadContext::read_equipment(pack, 6016, 201, "top").await?;
        let mdl = read_context.mdl;

        let quality = 0;
        let meshes = mdl.meshes(quality);
        let buffer_items = mdl.buffer_items(quality);

        let mut models = Vec::new();
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
            if color_table_data.len() != 0 {
                let color_table_tex = Texture::new(&renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float).await;
                textures.insert("ColorTable", color_table_tex);
            }

            let vs_bytes = include_bytes!("../shaders/shader.vert.spv");
            let fs_bytes = include_bytes!("../shaders/shader.frag.spv");

            let vs = Shader::new(
                &renderer,
                &vs_bytes[..],
                "main",
                hashmap! {"Locals" => ShaderBinding::new(0, ShaderBindingType::UniformBuffer)},
                hashmap! {
                    "Position" => 0,
                    "BoneWeight" => 1,
                    "BoneIndex" => 2,
                    "Normal" => 3,
                    "TexCoord" => 4,
                    "Bitangent" => 5,
                    "Color" => 6,
                },
            );
            let fs = Shader::new(
                &renderer,
                &fs_bytes[..],
                "main",
                hashmap! {
                    "Sampler" => ShaderBinding::new(1, ShaderBindingType::Sampler),
                    "Normal" => ShaderBinding::new(2, ShaderBindingType::Texture2D),
                    "ColorTable" => ShaderBinding::new(3, ShaderBindingType::Texture2D),
                    "Mask" => ShaderBinding::new(4, ShaderBindingType::Texture2D),
                },
                HashMap::new(),
            );
            let material = Material::new(&renderer, textures, vs, fs);

            models.push(Model::new(&renderer, mesh, material));
        }
        Ok(Self { models })
    }
}

impl Renderable for Character {
    fn render<'a>(&'a mut self, mut render_context: &mut RenderContext<'a>) {
        for model in &mut self.models {
            model.render(&mut render_context);
        }
    }
}
