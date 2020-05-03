use std::collections::HashMap;

use nalgebra::Matrix4;
use zerocopy::AsBytes;

use crate::Shader;
use crate::ShaderBindingType;
use crate::Texture;

pub struct Material {
    pub(crate) vertex_shader: Shader,
    pub(crate) fragment_shader: Shader,
    pub(crate) pipeline_layout: wgpu::PipelineLayout,

    textures: HashMap<&'static str, Texture>,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl Material {
    pub fn new(device: &wgpu::Device, textures: HashMap<&'static str, Texture>, vertex_shader: Shader, fragment_shader: Shader) -> Self {
        let vs_bindings = vertex_shader.wgpu_bindings(wgpu::ShaderStage::VERTEX);
        let fs_bindings = fragment_shader.wgpu_bindings(wgpu::ShaderStage::FRAGMENT);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &vs_bindings.into_iter().chain(fs_bindings.into_iter()).collect::<Vec<_>>(),
            label: None,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        Self {
            vertex_shader,
            fragment_shader,
            pipeline_layout,
            textures,
            bind_group_layout,
        }
    }

    pub fn bind_group(&self, device: &wgpu::Device, mvp: Matrix4<f32>) -> wgpu::BindGroup {
        let mvp_buf = device.create_buffer_with_data(mvp.as_slice().as_bytes(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let texture_views = self
            .textures
            .iter()
            .map(|(name, texture)| (name, texture.texture.create_default_view()))
            .collect::<HashMap<_, _>>();

        // TODO wip
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
        });

        let bindings = self
            .vertex_shader
            .bindings
            .iter()
            .chain(self.fragment_shader.bindings.iter())
            .map(|(binding_name, binding)| {
                let resource = match binding.binding_type {
                    ShaderBindingType::UniformBuffer => {
                        if *binding_name != "Locals" {
                            panic!() // TODO
                        }

                        wgpu::BindingResource::Buffer {
                            buffer: &mvp_buf,
                            range: 0..64,
                        }
                    }
                    ShaderBindingType::Texture2D => wgpu::BindingResource::TextureView(&texture_views.get(binding_name).unwrap()),
                    ShaderBindingType::Sampler => wgpu::BindingResource::Sampler(&sampler),
                };

                wgpu::Binding {
                    binding: binding.binding,
                    resource,
                }
            })
            .collect::<Vec<_>>();

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            bindings: &bindings,
            label: None,
        })
    }
}
