use crate::RenderContext;

pub trait Renderable: Sync + Send {
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>);
}
