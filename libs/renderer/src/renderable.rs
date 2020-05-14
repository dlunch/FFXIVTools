use crate::RenderContext;

pub trait Renderable {
    fn render<'a>(&'a mut self, render_context: &mut RenderContext<'a>);
}
