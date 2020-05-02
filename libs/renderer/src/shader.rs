use std::collections::HashMap;

pub enum ShaderBindingType {
    UniformBuffer,
    Texture2D,
    Sampler,
}

impl ShaderBindingType {
    pub fn wgpu_type(&self) -> wgpu::BindingType {
        match self {
            ShaderBindingType::UniformBuffer => wgpu::BindingType::UniformBuffer { dynamic: false },
            ShaderBindingType::Texture2D => wgpu::BindingType::SampledTexture {
                multisampled: false,
                component_type: wgpu::TextureComponentType::Float,
                dimension: wgpu::TextureViewDimension::D2,
            },
            ShaderBindingType::Sampler => wgpu::BindingType::Sampler { comparison: false },
        }
    }
}

pub struct ShaderBinding {
    binding: u32,
    binding_type: ShaderBindingType,
}

impl ShaderBinding {
    pub fn new(binding: u32, binding_type: ShaderBindingType) -> Self {
        Self { binding, binding_type }
    }

    pub fn wgpu_entry(&self, stage: wgpu::ShaderStage) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: stage,
            ty: self.binding_type.wgpu_type(),
        }
    }
}

pub struct Shader {
    pub(crate) module: wgpu::ShaderModule,
    pub(crate) entry: &'static str,
    bindings: HashMap<&'static str, ShaderBinding>,
}

impl Shader {
    pub fn new(device: &wgpu::Device, bytes: &[u32], entry: &'static str, bindings: HashMap<&'static str, ShaderBinding>) -> Self {
        let module = device.create_shader_module(bytes);

        Self { module, entry, bindings }
    }

    pub fn wgpu_bindings(&self, stage: wgpu::ShaderStage) -> Vec<wgpu::BindGroupLayoutEntry> {
        self.bindings.iter().map(|(_, x)| x.wgpu_entry(stage)).collect::<Vec<_>>()
    }
}
