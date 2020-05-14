use crate::UniformBuffer;

pub struct RenderContext<'a> {
    pub(crate) device: &'a wgpu::Device,
    pub(crate) render_pass: wgpu::RenderPass<'a>,
    pub(crate) mvp_buf: UniformBuffer,
}

impl<'a> RenderContext<'a> {
    pub fn new(device: &'a wgpu::Device, render_pass: wgpu::RenderPass<'a>, mvp_buf: UniformBuffer) -> Self {
        Self {
            device,
            render_pass,
            mvp_buf,
        }
    }
}
