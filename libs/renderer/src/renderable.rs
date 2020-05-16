use crate::RenderContext;

pub trait Renderable: Sync + Send {
    fn render<'a>(&'a mut self, render_context: &mut RenderContext<'a>);
}
