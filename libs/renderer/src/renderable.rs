use crate::UniformBuffer;

pub trait Renderable {
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>, mvp_buf: UniformBuffer);
}
