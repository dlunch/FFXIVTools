use alloc::vec::Vec;

use futures::lock::Mutex;
use nalgebra::Matrix4;
use raw_window_handle::HasRawWindowHandle;
use zerocopy::AsBytes;

use crate::{Camera, RenderContext, Scene, UniformBuffer};

type TextureUploadItem = (wgpu::Buffer, wgpu::Texture, usize, wgpu::Extent3d);

pub struct Renderer {
    pub(crate) device: wgpu::Device,
    pub(crate) empty_texture: wgpu::TextureView,
    pub(crate) mvp_buf: UniformBuffer,

    swap_chain: wgpu::SwapChain,
    queue: wgpu::Queue,

    depth_texture: wgpu::TextureView,
    texture_upload_queue: Mutex<Vec<TextureUploadItem>>,
    aspect_ratio: f32,
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
            format: wgpu::TextureFormat::Bgra8Unorm,
            width,
            height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let empty_texture = Self::create_empty_texture(&device, &queue).create_default_view();
        let mvp_buf = UniformBuffer::new(&device, 64);

        let depth_texture = device
            .create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d { width, height, depth: 1 },
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                label: None,
            })
            .create_default_view();

        Self {
            device,
            swap_chain,
            queue,
            texture_upload_queue: Mutex::new(Vec::new()),
            empty_texture,
            mvp_buf,
            depth_texture,
            aspect_ratio: (width as f32) / (height as f32),
        }
    }

    pub async fn render(&mut self, scene: &Scene<'_>) {
        let mvp = Self::get_mvp(&scene.camera, self.aspect_ratio);
        self.mvp_buf.write(&self.device, mvp.as_slice().as_bytes()).await.unwrap();

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.dequeue_texture_uploads(&mut command_encoder);

        let frame = self.swap_chain.get_next_texture().unwrap();
        {
            let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    clear_stencil: 0,
                }),
            });
            let mut render_context = RenderContext::new(render_pass);

            for model in &scene.models {
                model.render(&mut render_context);
            }
        }

        self.queue.submit(&[command_encoder.finish()]);
    }

    pub(crate) async fn enqueue_texture_upload(&self, buffer: wgpu::Buffer, texture: wgpu::Texture, bytes_per_row: usize, extent: wgpu::Extent3d) {
        let mut texture_upload_queue = self.texture_upload_queue.lock().await;
        texture_upload_queue.push((buffer, texture, bytes_per_row, extent));
    }

    fn dequeue_texture_uploads(&mut self, command_encoder: &mut wgpu::CommandEncoder) {
        let mut queue = Mutex::new(Vec::new());
        core::mem::swap(&mut self.texture_upload_queue, &mut queue);

        for (buffer, texture, bytes_per_row, extent) in queue.into_inner() {
            command_encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &buffer,
                    offset: 0,
                    bytes_per_row: bytes_per_row as u32,
                    rows_per_image: 0,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                extent,
            );
        }
    }

    fn get_mvp(camera: &Camera, aspect_ratio: f32) -> Matrix4<f32> {
        use core::f32::consts::PI;

        // nalgebra's perspective uses [-1, 1] NDC z range, so convert it to [0, 1].
        #[rustfmt::skip]
        let correction = nalgebra::Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        let projection = nalgebra::Matrix4::new_perspective(aspect_ratio, 45.0 * PI / 180.0, 1.0, 10.0);
        correction * projection * camera.view()
    }

    fn create_empty_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
        let extent = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        });

        let buffer = device.create_buffer_with_data(&[0, 0, 0, 0], wgpu::BufferUsage::COPY_SRC);
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        command_encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            extent,
        );

        queue.submit(&[command_encoder.finish()]);

        texture
    }
}
