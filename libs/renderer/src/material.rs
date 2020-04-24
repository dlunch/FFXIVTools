use nalgebra::Matrix4;
use zerocopy::AsBytes;

use crate::Texture;

pub struct Material {
    pub(crate) vs_module: wgpu::ShaderModule,
    pub(crate) fs_module: wgpu::ShaderModule,
    pub(crate) pipeline_layout: wgpu::PipelineLayout,
    pub(crate) bind_group: wgpu::BindGroup,

    texture: Texture,
    #[allow(dead_code)]
    uniform_buf: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl Material {
    pub fn new(device: &wgpu::Device, texture: Texture, vs_bytes: &[u32], fs_bytes: &[u32]) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
            label: None,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vs_module = device.create_shader_module(vs_bytes);
        let fs_module = device.create_shader_module(fs_bytes);

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.texture.create_default_view()),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        Self {
            vs_module,
            fs_module,
            pipeline_layout,
            bind_group,
            texture,
            bind_group_layout,
            uniform_buf,
        }
    }

    pub fn set_mvp(&mut self, device: &wgpu::Device, mvp: Matrix4<f32>) {
        // TODO use buffer upload
        let uniform_buf = device.create_buffer_with_data(mvp.as_slice().as_bytes(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.texture.texture.create_default_view()),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.texture.sampler),
                },
            ],
            label: None,
        });
    }
}
