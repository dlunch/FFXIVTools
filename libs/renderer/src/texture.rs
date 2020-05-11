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
    pub(crate) fn bytes_per_row(&self) -> u32 {
        match self {
            TextureFormat::Rgba8Unorm => 4,
            TextureFormat::Rgba16Float => 8,
        }
    }
}

pub struct Texture {
    pub(crate) texture: wgpu::Texture,
    format: TextureFormat,
    extent: wgpu::Extent3d,
    buffer: Option<wgpu::Buffer>,
}

impl Texture {
    pub fn new(renderer: &Renderer, width: u32, height: u32, texels: &[u8], format: TextureFormat) -> Self {
        let extent = wgpu::Extent3d { width, height, depth: 1 };
        let texture = renderer.device.create_texture(&wgpu::TextureDescriptor {
            size: extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format.wgpu_type(),
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        });

        let buffer = Some(renderer.device.create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC));

        Self {
            texture,
            format,
            extent,
            buffer,
        }
    }

    pub(crate) fn prepare(&mut self, command_encoder: &mut wgpu::CommandEncoder) {
        if let Some(buffer) = &self.buffer {
            command_encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer,
                    offset: 0,
                    bytes_per_row: self.format.bytes_per_row() * self.extent.width,
                    rows_per_image: 0,
                },
                wgpu::TextureCopyView {
                    texture: &self.texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                self.extent,
            );
            self.buffer = None;
        }
    }
}
