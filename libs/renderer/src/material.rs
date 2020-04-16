use crate::Texture;

pub struct Material {
    pub(crate) texture: Texture,
    pub(crate) vs_module: wgpu::ShaderModule,
    pub(crate) fs_module: wgpu::ShaderModule,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) pipeline_layout: wgpu::PipelineLayout,
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

        Self {
            texture,
            vs_module,
            fs_module,
            bind_group_layout,
            pipeline_layout,
        }
    }
}
