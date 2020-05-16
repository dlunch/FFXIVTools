use crate::Renderer;

pub enum TextureFormat {
    Rgba8Unorm,
    Bgra8Unorm,
    Rgba16Float,
}

impl TextureFormat {
    pub(crate) fn wgpu_type(&self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            TextureFormat::Rgba16Float => wgpu::TextureFormat::Rgba16Float,
        }
    }
    pub(crate) fn bytes_per_row(&self) -> usize {
        match self {
            TextureFormat::Rgba8Unorm => 4,
            TextureFormat::Bgra8Unorm => 4,
            TextureFormat::Rgba16Float => 8,
        }
    }
}

pub enum CompressedTextureFormat {
    BC1,
    BC2,
    BC3,
}

impl CompressedTextureFormat {
    pub(crate) fn decoded_format(&self) -> TextureFormat {
        match self {
            CompressedTextureFormat::BC1 => TextureFormat::Rgba8Unorm,
            CompressedTextureFormat::BC2 => TextureFormat::Rgba8Unorm,
            CompressedTextureFormat::BC3 => TextureFormat::Rgba8Unorm,
        }
    }
}

pub struct Texture {
    pub(crate) texture_view: wgpu::TextureView,
}

impl Texture {
    pub async fn new(renderer: &Renderer, width: u32, height: u32, texels: &[u8], format: TextureFormat) -> Self {
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

        let texture_view = texture.create_default_view();
        let buffer = renderer.device.create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC);
        renderer
            .enqueue_texture_upload(buffer, texture, format.bytes_per_row() * extent.width as usize, extent)
            .await;

        Self { texture_view }
    }

    pub async fn new_compressed(renderer: &Renderer, width: u32, height: u32, data: &[u8], format: CompressedTextureFormat) -> Self {
        let uncompressed = Self::decode_texture(data, width, height, &format);

        Self::new(renderer, width, height, &uncompressed, format.decoded_format()).await
    }

    fn decode_texture(data: &[u8], width: u32, height: u32, format: &CompressedTextureFormat) -> Vec<u8> {
        let result_size = (width as usize) * (height as usize) * 4; // RGBA
        let mut result = vec![0; result_size];

        let format = match format {
            CompressedTextureFormat::BC1 => squish::Format::Bc1,
            CompressedTextureFormat::BC2 => squish::Format::Bc2,
            CompressedTextureFormat::BC3 => squish::Format::Bc3,
        };
        format.decompress(data, width as usize, height as usize, result.as_mut());

        result
    }
}
