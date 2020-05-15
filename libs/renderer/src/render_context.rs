use crate::{Renderer, UniformBuffer};

pub struct RenderContext<'a> {
    pub(crate) renderer: &'a Renderer,
    pub(crate) render_pass: wgpu::RenderPass<'a>,
    pub(crate) mvp_buf: UniformBuffer,
}

impl<'a> RenderContext<'a> {
    pub fn new(renderer: &'a Renderer, render_pass: wgpu::RenderPass<'a>, mvp_buf: UniformBuffer) -> Self {
        Self {
            renderer,
            render_pass,
            mvp_buf,
        }
    }
}
