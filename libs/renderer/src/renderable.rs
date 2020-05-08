use crate::UniformBuffer;

pub trait Renderable {
    fn render(&mut self, device: &wgpu::Device, command_encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainOutput, mvp_buf: UniformBuffer);
}
