use std::collections::HashMap;

use futures::{future, FutureExt};
use maplit::hashmap;

use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl, Mtrl, MtrlParameterType, Tex, TextureType};
use renderer::{
    Material, Mesh, Model, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture, TextureFormat, VertexFormat, VertexFormatItem, VertexItemType,
};
use sqpack_reader::{Package, Result};

pub struct Character {
    pub model: Model,
}

impl Character {
    pub async fn new(pack: &dyn Package, renderer: &Renderer) -> Result<Self> {
        // WIP
        let mdl = Mdl::new(pack, "chara/equipment/e6016/model/c0201e6016_top.mdl").await?;
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
            &renderer.device,
            mesh[mesh_index].buffers.as_ref(),
            &strides,
            mesh[mesh_index].indices,
            mesh[0].mesh_info.index_count as usize,
            vertex_formats,
        );

        let material_file = convert_material_filename(&mdl.material_files()[mesh_index]);
        let mtrl = Mtrl::new(pack, material_file).await?;

        let texture_files = mtrl.texture_files();
        let mut textures = future::join_all(mtrl.parameters().iter().map(|parameter| {
            Tex::new(pack, &texture_files[parameter.texture_index as usize]).map(move |tex| {
                let tex = tex?;
                Ok((
                    convert_texture_name(parameter.parameter_type),
                    Texture::new(
                        &renderer.device,
                        tex.width() as u32,
                        tex.height() as u32,
                        decode_texture(tex, 0).as_ref(),
                        TextureFormat::Rgba8Unorm,
                    ),
                ))
            })
        }))
        .await
        .into_iter()
        .collect::<Result<HashMap<_, _>>>()?;

        let color_table_data = mtrl.color_table();
        let color_table_tex = Texture::new(&renderer.device, 4, 16, color_table_data, TextureFormat::Rgba16Float);
        textures.insert("ColorTable", color_table_tex);

        let vs_bytes = include_bytes!("../shaders/shader.vert.spv");
        let fs_bytes = include_bytes!("../shaders/shader.frag.spv");

        let vs = Shader::new(
            &renderer.device,
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
            &renderer.device,
            &fs_bytes[..],
            "main",
            hashmap! {
                "Normal" => ShaderBinding::new(1, ShaderBindingType::Texture2D),
                "s_Color" => ShaderBinding::new(2, ShaderBindingType::Sampler),
                "ColorTable" => ShaderBinding::new(3, ShaderBindingType::Texture2D),
                "Mask" => ShaderBinding::new(4, ShaderBindingType::Texture2D),
            },
            HashMap::new(),
        );
        let material = Material::new(&renderer.device, textures, vs, fs);

        let model = Model::new(&renderer.device, mesh, material);

        Ok(Self { model })
    }
}

fn decode_texture(tex: Tex, mipmap_index: u16) -> Vec<u8> {
    let raw = tex.data(mipmap_index);
    let result_size = (tex.width() as usize) * (tex.height() as usize) * 4; // RGBA
    let mut result = vec![0; result_size];

    let format = match tex.texture_type() {
        TextureType::DXT1 => squish::Format::Bc1,
        TextureType::DXT3 => squish::Format::Bc2,
        TextureType::DXT5 => squish::Format::Bc3,
        _ => panic!(),
    };
    format.decompress(raw, tex.width() as usize, tex.height() as usize, result.as_mut());

    result
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

fn convert_material_filename(material_file: &str) -> String {
    if material_file.chars().nth(9).unwrap() == 'b' {
        "".to_owned() // TODO body material
    } else {
        let variant_id = 1; // TODO
        let equipment_id = 6016;

        format!("chara/equipment/e{:04}/material/v{:04}{}", equipment_id, variant_id, material_file)
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
