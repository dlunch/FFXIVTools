use raw_window_handle::HasRawWindowHandle;
use zerocopy::{AsBytes, FromBytes};

use renderer::{Material, Mesh, Model, Renderer, Texture, TextureFormat, VertexFormat, VertexFormatItem, VertexItemType};

enum ShaderStage {
    Vertex,
    Fragment,
}

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

        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture = Texture::new(
            &renderer.device,
            &mut renderer.command_encoder,
            size,
            size,
            &texels,
            TextureFormat::Rgba8Unorm,
        );

        let vs = Self::load_glsl(include_str!("shader.vert"), ShaderStage::Vertex);
        let fs = Self::load_glsl(include_str!("shader.frag"), ShaderStage::Fragment);
        let material = Material::new(&renderer.device, texture, vs.as_binary(), fs.as_binary());

        let model = Model::new(&renderer.device, mesh, material);

        Self { model, renderer }
    }

    pub fn redraw(&mut self) {
        self.renderer.render(&self.model)
    }

    fn load_glsl(code: &str, stage: ShaderStage) -> shaderc::CompilationArtifact {
        let ty = match stage {
            ShaderStage::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStage::Fragment => shaderc::ShaderKind::Fragment,
        };

        let mut compiler = shaderc::Compiler::new().unwrap();
        compiler.compile_into_spirv(code, ty, "shader.glsl", "main", None).unwrap()
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

fn create_texels(size: usize) -> Vec<u8> {
    use std::iter;

    (0..size * size)
        .flat_map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            iter::once(0xFF - (count * 5) as u8)
                .chain(iter::once(0xFF - (count * 15) as u8))
                .chain(iter::once(0xFF - (count * 50) as u8))
                .chain(iter::once(1))
        })
        .collect()
}
