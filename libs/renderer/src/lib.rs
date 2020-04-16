mod material;
mod mesh;
mod model;
mod texture;

pub use material::Material;
pub use mesh::Mesh;
pub use model::Model;
pub use texture::Texture;

use raw_window_handle::HasRawWindowHandle;

pub struct Renderer {
    pub device: wgpu::Device,
    pub command_encoder: wgpu::CommandEncoder,
    swap_chain: wgpu::SwapChain,
    queue: wgpu::Queue,
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

        let command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Self {
            device,
            swap_chain,
            queue,
            command_encoder,
        }
    }

    pub fn render(&mut self, model: &Model) {
        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        std::mem::swap(&mut command_encoder, &mut self.command_encoder);

        let frame = self.swap_chain.get_next_texture().unwrap();
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
            rpass.set_pipeline(&model.pipeline);
            rpass.set_bind_group(0, &model.bind_group, &[]);
            rpass.set_index_buffer(&model.mesh.index, 0, 0);
            rpass.set_vertex_buffer(0, &model.mesh.vertex, 0, 0);
            rpass.draw_indexed(0..model.mesh.index_count as u32, 0, 0..1);
        }

        self.queue.submit(&[command_encoder.finish()]);
    }
}
