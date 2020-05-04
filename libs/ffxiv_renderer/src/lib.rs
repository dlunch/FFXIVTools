use maplit::hashmap;
use nalgebra::Point3;
use raw_window_handle::HasRawWindowHandle;
use shaderc::ShaderKind;

use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl, Tex, TextureType};
use renderer::{
    Camera, Material, Mesh, Model, Renderer, Shader, ShaderBinding, ShaderBindingType, Texture, TextureFormat, VertexFormat, VertexFormatItem,
    VertexItemType,
};
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct FFXIVRenderer {
    model: Model,
    renderer: Renderer,
}

impl FFXIVRenderer {
    pub async fn new<W: HasRawWindowHandle>(window: &W, width: u32, height: u32) -> Self {
        let renderer = Renderer::new(window, width, height).await;

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
            VertexFormat::new(vec![VertexFormatItem::new(0, convert_type(position.item_type), position.offset as usize)]),
            VertexFormat::new(vec![VertexFormatItem::new(
                1,
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

        let tex = Tex::new(&pack, "chara/human/c0101/obj/body/b0001/texture/c0101b0001_d.tex")
            .await
            .unwrap();

        // TODO hide command_encoder detail
        let mut command_encoder = renderer.create_command_encoder();
        let texture = Texture::new(
            &renderer.device,
            &mut command_encoder,
            tex.width() as u32,
            tex.height() as u32,
            decode_texture(tex, 0).as_ref(),
            TextureFormat::Rgba8Unorm,
        );

        renderer.queue.submit(&[command_encoder.finish()]);

        let textures = hashmap! {
            "t_Color" => texture,
        };

        let vs_bytes = Self::load_glsl(include_str!("shader.vert"), ShaderKind::Vertex);
        let fs_bytes = Self::load_glsl(include_str!("shader.frag"), ShaderKind::Fragment);
        let vs = Shader::new(
            &renderer.device,
            vs_bytes.as_binary(),
            "main",
            hashmap! {"Locals" => ShaderBinding::new(0, ShaderBindingType::UniformBuffer)},
        );
        let fs = Shader::new(
            &renderer.device,
            fs_bytes.as_binary(),
            "main",
            hashmap! {
                "t_Color" => ShaderBinding::new(1, ShaderBindingType::Texture2D),
                "s_Color" => ShaderBinding::new(2, ShaderBindingType::Sampler)
            },
        );
        let material = Material::new(&renderer.device, textures, vs, fs);

        let model = Model::new(&renderer.device, mesh, material);

        Self { model, renderer }
    }

    pub async fn redraw(&mut self) {
        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        self.renderer.render(&mut self.model, &camera).await
    }

    fn load_glsl(code: &str, stage: ShaderKind) -> shaderc::CompilationArtifact {
        let mut compiler = shaderc::Compiler::new().unwrap();
        compiler.compile_into_spirv(code, stage, "shader.glsl", "main", None).unwrap()
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
