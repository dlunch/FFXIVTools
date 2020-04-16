mod material;
mod mesh;
mod model;
mod texture;

pub use material::Material;
pub use mesh::Mesh;
pub use model::Model;
pub use texture::Texture;

use raw_window_handle::HasRawWindowHandle;

use zerocopy::{AsBytes, FromBytes};

pub struct Renderer {
    device: wgpu::Device,
    swap_chain: wgpu::SwapChain,
    queue: wgpu::Queue,
    command_encoder: wgpu::CommandEncoder,
    example: Example,
}

impl Renderer {
    pub async fn new<W: HasRawWindowHandle>(window: &W, width: u32, height: u32) -> Self {
        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let example = Example::init(&device, &mut command_encoder);
        queue.submit(&[command_encoder.finish()]);

        let command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Self {
            device,
            swap_chain,
            queue,
            command_encoder,
            example,
        }
    }

    pub fn redraw(&mut self) {
        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        std::mem::swap(&mut command_encoder, &mut self.command_encoder);

        let frame = self.swap_chain.get_next_texture().unwrap();
        self.example.render(&frame, &mut command_encoder);
        self.queue.submit(&[command_encoder.finish()]);
    }
}

struct Example {
    model: Model,
}

impl Example {
    fn init(device: &wgpu::Device, mut command_encoder: &mut wgpu::CommandEncoder) -> Self {
        use std::mem;

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<Vertex>();
        let (vertex_data, index_data) = create_vertices();
        let mesh = mesh::Mesh::new(&device, vertex_data.as_bytes(), vertex_size, index_data.as_bytes(), index_data.len());

        // Create the texture
        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture = texture::Texture::from_texels(&device, &mut command_encoder, size, size, &texels);
        let material = Material::new(&device, texture);

        let model = model::Model::new(&device, mesh, material);

        Example { model }
    }

    fn render(&mut self, frame: &wgpu::SwapChainOutput, command_encoder: &mut wgpu::CommandEncoder) {
        {
            let mut rpass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.model.pipeline);
            rpass.set_bind_group(0, &self.model.bind_group, &[]);
            rpass.set_index_buffer(&self.model.mesh.index, 0, 0);
            rpass.set_vertex_buffer(0, &self.model.mesh.vertex, 0, 0);
            rpass.draw_indexed(0..self.model.mesh.index_count as u32, 0, 0..1);
        }
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
