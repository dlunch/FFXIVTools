pub struct Shader {
    pub(crate) module: wgpu::ShaderModule,
    pub(crate) entry: &'static str,
}

impl Shader {
    pub fn new(device: &wgpu::Device, bytes: &[u32], entry: &'static str) -> Self {
        let module = device.create_shader_module(bytes);

        Self { module, entry }
    }
}
