mod buffer;
mod camera;
mod material;
mod mesh;
mod model;
mod shader;
mod texture;
mod vertex_format;

pub use buffer::Buffer;
pub use camera::Camera;
pub use material::Material;
pub use mesh::Mesh;
pub use model::Model;
pub use shader::{Shader, ShaderBinding, ShaderBindingType};
pub use texture::{Texture, TextureFormat};
pub use vertex_format::{VertexFormat, VertexFormatItem, VertexItemType};

use nalgebra::Matrix4;
use raw_window_handle::HasRawWindowHandle;
use zerocopy::AsBytes;

pub struct Renderer {
    pub device: wgpu::Device,
    swap_chain: wgpu::SwapChain,
    pub queue: wgpu::Queue,
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

        Self { device, swap_chain, queue }
    }

    // TODO hide command_encoder detail
    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    pub async fn render(&mut self, model: &mut Model, camera: &Camera) {
        let mvp = Self::get_mvp(camera, 1024.0 / 768.0);
        let mut mvp_buf = Buffer::new(&self.device, 64);
        mvp_buf.write(&self.device, mvp.as_slice().as_bytes()).await.unwrap();

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let frame = self.swap_chain.get_next_texture().unwrap();
        model.render(&self.device, &mut command_encoder, &frame, mvp_buf);

        self.queue.submit(&[command_encoder.finish()]);
    }

    fn get_mvp(camera: &Camera, aspect_ratio: f32) -> Matrix4<f32> {
        use core::f32::consts::PI;

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
