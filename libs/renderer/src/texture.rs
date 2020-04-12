pub struct Texture {
    texture: wgpu::Texture,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_texels(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, width: u32, height: u32, texels: &[u8]) -> Self {
        let texture_extent = wgpu::Extent3d { width, height, depth: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        });

        let temp_buf = device.create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buf,
                offset: 0,
                bytes_per_row: 4 * width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            texture_extent,
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
        });

        Self { texture, sampler }
    }

    pub fn view(&self) -> wgpu::TextureView {
        self.texture.create_default_view()
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
