pub struct RenderContext<'a> {
    pub(crate) render_pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn new(render_pass: wgpu::RenderPass<'a>) -> Self {
        Self { render_pass }
    }
}
