use std::sync::Arc;

use crate::Renderer;

pub enum TextureFormat {
    Rgba8Unorm,
    Rgba16Float,
}

impl TextureFormat {
    pub(crate) fn wgpu_type(&self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Rgba16Float => wgpu::TextureFormat::Rgba16Float,
        }
    }
    pub(crate) fn bytes_per_row(&self) -> usize {
        match self {
            TextureFormat::Rgba8Unorm => 4,
            TextureFormat::Rgba16Float => 8,
        }
    }
}

pub struct Texture {
    pub(crate) texture: Arc<wgpu::Texture>,
}

impl Texture {
    pub async fn new(renderer: &Renderer, width: u32, height: u32, texels: &[u8], format: TextureFormat) -> Self {
        let extent = wgpu::Extent3d { width, height, depth: 1 };
        let texture = Arc::new(renderer.device.create_texture(&wgpu::TextureDescriptor {
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format.wgpu_type(),
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        }));

        let buffer = renderer.device.create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC);
        renderer
            .enqueue_texture_upload(buffer, texture.clone(), format.bytes_per_row() * extent.width as usize, extent)
            .await;

        Self { texture }
    }
}
