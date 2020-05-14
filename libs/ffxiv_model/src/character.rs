use std::collections::HashMap;

use futures::{future, FutureExt};
use maplit::hashmap;

use ffxiv_parser::{BufferItemType, BufferItemUsage, MtrlParameterType, TextureType};
use renderer::{
    CompressedTextureFormat, Material, Mesh, Model, RenderContext, Renderable, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture,
    TextureFormat, VertexFormat, VertexFormatItem, VertexItemType,
};
use sqpack_reader::{Package, Result};

use crate::model_read_context::ModelReadContext;

pub struct Character {
    model: Model,
}

impl Character {
    pub async fn new(pack: &dyn Package, renderer: &Renderer) -> Result<Self> {
        // WIP
        let read_context = ModelReadContext::read_equipment(pack, 6016, 201, "top").await?;
        let mdl = read_context.mdl;

        let mesh = mdl.meshes(0);
        let buffer_items = mdl.buffer_items(0).collect::<Vec<_>>();
        let mesh_index = 0;

        let vertex_formats = (0..mesh[mesh_index].mesh_info.buffer_count as usize)
            .map(|buffer_index| {
                let buffer_items = buffer_items[mesh_index].items().filter(move |x| x.buffer as usize == buffer_index);
                VertexFormat::new(
                    buffer_items
                        .map(|x| VertexFormatItem::new(convert_usage(x.usage), convert_type(x.item_type), x.offset as usize))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let strides = (0..mesh[mesh_index].mesh_info.buffer_count as usize)
            .map(|i| mesh[mesh_index].mesh_info.strides[i] as usize)
            .collect::<Vec<_>>();

        let mesh = Mesh::new(
            &renderer,
            mesh[mesh_index].buffers.as_ref(),
            &strides,
            mesh[mesh_index].indices,
            mesh[0].mesh_info.index_count as usize,
            vertex_formats,
        );

        let (mtrl, texs) = &read_context.mtrls[0];
        let mut textures = future::join_all(mtrl.parameters().iter().map(|parameter| {
            let tex_name = convert_texture_name(parameter.parameter_type);
            let tex = &texs[parameter.texture_index as usize];
            Texture::new_compressed(
                &renderer,
                tex.width() as u32,
                tex.height() as u32,
                tex.data(0),
                convert_texture_format(tex.texture_type()),
            )
            .map(move |x| (tex_name, x))
        }))
        .await
        .into_iter()
        .collect::<HashMap<_, _>>();

        let color_table_data = mtrl.color_table();
        let color_table_tex = Texture::new(&renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float).await;
        textures.insert("ColorTable", color_table_tex);

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

        let model = Model::new(&renderer, mesh, material);

        Ok(Self { model })
    }
}

impl Renderable for Character {
    fn render<'a>(&'a mut self, mut render_context: &mut RenderContext<'a>) {
        self.model.render(&mut render_context);
    }
}

fn convert_texture_format(texture_type: TextureType) -> CompressedTextureFormat {
    match texture_type {
        TextureType::DXT1 => CompressedTextureFormat::BC1,
        TextureType::DXT3 => CompressedTextureFormat::BC2,
        TextureType::DXT5 => CompressedTextureFormat::BC3,
        _ => panic!(),
    }
}

fn convert_type(item_type: BufferItemType) -> VertexItemType {
    match item_type {
        BufferItemType::UByte4 => VertexItemType::UByte4,
        BufferItemType::UByte4n => VertexItemType::UByte4,
        BufferItemType::Float2 => VertexItemType::Float2,
        BufferItemType::Float3 => VertexItemType::Float3,
        BufferItemType::Float4 => VertexItemType::Float4,
        BufferItemType::Half2 => VertexItemType::Half2,
        BufferItemType::Half4 => VertexItemType::Half4,
        _ => panic!(),
    }
}

fn convert_usage(usage: BufferItemUsage) -> &'static str {
    match usage {
        BufferItemUsage::Position => "Position",
        BufferItemUsage::BoneWeight => "BoneWeight",
        BufferItemUsage::BoneIndex => "BoneIndex",
        BufferItemUsage::Normal => "Normal",
        BufferItemUsage::TexCoord => "TexCoord",
        BufferItemUsage::Tangent => "Tangent",
        BufferItemUsage::Bitangent => "Bitangent",
        BufferItemUsage::Color => "Color",
    }
}

fn convert_texture_name(parameter_type: MtrlParameterType) -> &'static str {
    match parameter_type {
        MtrlParameterType::Normal => "Normal",
        MtrlParameterType::Mask => "Mask",
        MtrlParameterType::Diffuse => "Diffuse",
        MtrlParameterType::Specular => "Specular",
        MtrlParameterType::Catchlight => "Catchlight",
    }
}
