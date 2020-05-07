use std::collections::HashMap;

use maplit::hashmap;

use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl, Mtrl, MtrlParameterType, Tex, TextureType};
use renderer::{
    Material, Mesh, Model, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture, TextureFormat, VertexFormat, VertexFormatItem, VertexItemType,
};
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct Character {
    pub model: Model,
}

impl Character {
    pub async fn new(renderer: &Renderer) -> Self {
        // WIP
        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider).unwrap();

        let mdl = Mdl::new(&pack, "chara/equipment/e6016/model/c0201e6016_top.mdl").await.unwrap();
        let mesh = mdl.meshes(0);
        let buffer_items = mdl.buffer_items(0).collect::<Vec<_>>();
        let mesh_index = 0;

        let position = buffer_items[mesh_index].items().find(|x| x.usage == BufferItemUsage::Position).unwrap();
        let tex_coord = buffer_items[mesh_index].items().find(|x| x.usage == BufferItemUsage::TexCoord).unwrap();

        let vertex_formats = vec![
            VertexFormat::new(vec![VertexFormatItem::new(
                "Position",
                convert_type(position.item_type),
                position.offset as usize,
            )]),
            VertexFormat::new(vec![VertexFormatItem::new(
                "TexCoord",
                convert_type(tex_coord.item_type),
                tex_coord.offset as usize,
            )]),
        ];

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

        let mut textures = HashMap::new();

        let material_file = convert_material_filename(&mdl.material_files()[mesh_index]);
        let mtrl = Mtrl::new(&pack, material_file).await.unwrap();
        let texture_files = mtrl.texture_files();
        for parameter in mtrl.parameters() {
            if parameter.parameter_type == MtrlParameterType::Normal {
                let texture_file = &texture_files[parameter.texture_index as usize];
                let tex = Tex::new(&pack, texture_file).await.unwrap();

                let texture = Texture::new(
                    &renderer.device,
                    tex.width() as u32,
                    tex.height() as u32,
                    decode_texture(tex, 0).as_ref(),
                    TextureFormat::Rgba8Unorm,
                );

                textures.insert("t_Normal", texture);
            }
        }

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
            hashmap! { "Position" => 0, "TexCoord" => 1},
        );
        let fs = Shader::new(
            &renderer.device,
            &fs_bytes[..],
            "main",
            hashmap! {
                "t_Normal" => ShaderBinding::new(1, ShaderBindingType::Texture2D),
                "s_Color" => ShaderBinding::new(2, ShaderBindingType::Sampler),
                "ColorTable" => ShaderBinding::new(3, ShaderBindingType::Texture2D),
            },
            HashMap::new(),
        );
        let material = Material::new(&renderer.device, textures, vs, fs);

        let model = Model::new(&renderer.device, mesh, material);

        Self { model }
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
