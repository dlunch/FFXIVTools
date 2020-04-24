mod camera;
mod material;
mod mesh;
mod model;
mod texture;
mod vertex_format;

pub use camera::Camera;
pub use material::Material;
pub use mesh::Mesh;
pub use model::Model;
pub use texture::{Texture, TextureFormat};
pub use vertex_format::{VertexFormat, VertexFormatItem, VertexItemType};

use nalgebra::Matrix4;
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

    pub fn render(&mut self, model: &mut Model, camera: &Camera) {
        let mvp = Self::get_mvp(camera, 1024.0 / 768.0);
        model.set_mvp(&self.device, mvp);

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        std::mem::swap(&mut command_encoder, &mut self.command_encoder);

        let frame = self.swap_chain.get_next_texture().unwrap();
        model.render(&mut command_encoder, &frame);

        self.queue.submit(&[command_encoder.finish()]);
    }

    fn get_mvp(camera: &Camera, aspect_ratio: f32) -> Matrix4<f32> {
        use std::f32::consts::PI;

        // nalgebra's perspective uses [-1, 1] NDC z range, so convert it to [0, 1].
        #[rustfmt::skip]
        let correction: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        let projection = nalgebra::Matrix4::new_perspective(aspect_ratio, 45.0 * PI / 180.0, 1.0, 10.0);
        correction * projection * camera.view()
    }
}
