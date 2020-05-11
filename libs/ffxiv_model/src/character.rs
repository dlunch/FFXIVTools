use std::collections::HashMap;

use futures::{future, FutureExt};
use maplit::hashmap;

use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl, Mtrl, MtrlParameterType, Tex, TextureType};
use renderer::{
    Material, Mesh, Model, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture, TextureFormat, VertexFormat, VertexFormatItem, VertexItemType,
};
use sqpack_reader::{Package, Result};

struct ModelReadContext {
    pub mdl: Mdl,
    pub mtrls: Vec<(Mtrl, Vec<Tex>)>,
}

impl ModelReadContext {
    pub async fn read_equipment(pack: &dyn Package, model_id: u16, body_id: u16, model_part: &'static str) -> Result<Self> {
        let mdl_filename = format!(
            "chara/equipment/e{model_id:04}/model/c{body_id:04}e{model_id:04}_{model_part}.mdl",
            model_id = model_id,
            body_id = body_id,
            model_part = model_part
        );
        let mdl = Mdl::new(pack, &mdl_filename).await?;

        let mtrls = future::join_all(mdl.material_files().iter().map(|material_file| {
            let material_file = Self::convert_equipment_material_filename(&material_file);
            Mtrl::new(pack, material_file).then(|mtrl| async {
                let mtrl = mtrl?;
                let texture_files = mtrl.texture_files();

                let texs = future::join_all(texture_files.iter().map(|texture_file| Tex::new(pack, texture_file)))
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?;

                Ok((mtrl, texs))
            })
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        Ok(Self { mdl, mtrls })
    }

    fn convert_equipment_material_filename(material_file: &str) -> String {
        if material_file.chars().nth(9).unwrap() == 'b' {
            let body_id = 201;
            let body_type = 1;
            let variant_id = 1;
            format!(
                "chara/human/c{body_id:04}/obj/body/b{body_type:04}/material/v{variant_id:04}/mt_c{body_id:04}b{body_type:04}{path}",
                body_id = body_id,
                body_type = body_type,
                variant_id = variant_id,
                path = &material_file[14..]
            )
        } else {
            let variant_id = 1; // TODO
            let equipment_id = 6016;

            format!("chara/equipment/e{:04}/material/v{:04}{}", equipment_id, variant_id, material_file)
        }
    }
}

pub struct Character {
    pub model: Model,
}

impl Character {
    pub async fn new(pack: &dyn Package, mut renderer: &mut Renderer) -> Result<Self> {
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

        let mut textures = mtrl
            .parameters()
            .iter()
            .map(|parameter| {
                let tex = &texs[parameter.texture_index as usize];
                (
                    convert_texture_name(parameter.parameter_type),
                    Texture::new(
                        &mut renderer,
                        tex.width() as u32,
                        tex.height() as u32,
                        decode_texture(tex, 0).as_ref(),
                        TextureFormat::Rgba8Unorm,
                    ),
                )
            })
            .collect::<HashMap<_, _>>();

        let color_table_data = mtrl.color_table();
        let color_table_tex = Texture::new(&mut renderer, 4, 16, color_table_data, TextureFormat::Rgba16Float);
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

fn decode_texture(tex: &Tex, mipmap_index: u16) -> Vec<u8> {
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
