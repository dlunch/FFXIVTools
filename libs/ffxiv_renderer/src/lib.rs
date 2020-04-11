use raw_window_handle::HasRawWindowHandle;
use renderer::Renderer;

pub struct FFXIVRenderer {
    renderer: Renderer,
}

impl FFXIVRenderer {
    pub async fn new<W: HasRawWindowHandle>(window: &W) -> Self {
        Self {
            renderer: Renderer::new(window).await,
        }
    }
    pub fn redraw(&mut self) {
        self.renderer.redraw()
    }
}
