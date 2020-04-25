use nalgebra::Point3;
use raw_window_handle::HasRawWindowHandle;
use shaderc::ShaderKind;
use zerocopy::{AsBytes, FromBytes};

use ffxiv_parser::{Tex, TextureType};
use renderer::{Camera, Material, Mesh, Model, Renderer, Texture, TextureFormat, VertexFormat, VertexFormatItem, VertexItemType};
use sqpack_reader::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

pub struct FFXIVRenderer {
    model: Model,
    renderer: Renderer,
}

impl FFXIVRenderer {
    pub async fn new<W: HasRawWindowHandle>(window: &W, width: u32, height: u32) -> Self {
        let mut renderer = Renderer::new(window, width, height).await;

        // Create the vertex and index buffers
        let vertex_size = std::mem::size_of::<Vertex>();
        let (vertex_data, index_data) = create_vertices();
        let vertex_format = VertexFormat::new(vec![
            VertexFormatItem::new(VertexItemType::Float4, 0),
            VertexFormatItem::new(VertexItemType::Float2, 16),
        ]);

        let mesh = Mesh::new(
            &renderer.device,
            vertex_data.as_bytes(),
            vertex_size,
            index_data.as_bytes(),
            index_data.len(),
            vertex_format,
        );

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider).unwrap();

        let tex = Tex::new(&pack, "chara/human/c0101/obj/body/b0001/texture/c0101b0001_d.tex")
            .await
            .unwrap();

        let texture = Texture::new(
            &renderer.device,
            &mut renderer.command_encoder,
            tex.width() as u32,
            tex.height() as u32,
            decode_texture(tex, 0).as_ref(),
            TextureFormat::Rgba8Unorm,
        );

        let vs = Self::load_glsl(include_str!("shader.vert"), ShaderKind::Vertex);
        let fs = Self::load_glsl(include_str!("shader.frag"), ShaderKind::Fragment);
        let material = Material::new(&renderer.device, texture, vs.as_binary(), fs.as_binary());

        let model = Model::new(&renderer.device, mesh, material);

        Self { model, renderer }
    }

    pub fn redraw(&mut self) {
        let camera = Camera::new(Point3::new(1.5f32, -5.0, 3.0), Point3::new(0.0, 0.0, 0.0));
        self.renderer.render(&mut self.model, &camera)
    }

    fn load_glsl(code: &str, stage: ShaderKind) -> shaderc::CompilationArtifact {
        let mut compiler = shaderc::Compiler::new().unwrap();
        compiler.compile_into_spirv(code, stage, "shader.glsl", "main", None).unwrap()
    }
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1], [0, 0]),
        vertex([1, -1, 1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1], [1, 0]),
        vertex([1, 1, -1], [0, 0]),
        vertex([1, -1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        vertex([1, -1, -1], [0, 0]),
        vertex([1, 1, -1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, 1, 1], [0, 0]),
        vertex([-1, 1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        vertex([1, 1, -1], [1, 0]),
        vertex([-1, 1, -1], [0, 0]),
        vertex([-1, 1, 1], [0, 1]),
        vertex([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1], [0, 0]),
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, -1, -1], [1, 1]),
        vertex([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
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
