use crate::UniformBuffer;

pub trait Renderable {
    fn prepare(&mut self, command_encoder: &mut wgpu::CommandEncoder);
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>, mvp_buf: UniformBuffer);
}
