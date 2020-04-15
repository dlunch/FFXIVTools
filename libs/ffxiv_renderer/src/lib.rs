use raw_window_handle::HasRawWindowHandle;
use renderer::Renderer;

pub struct FFXIVRenderer {
    renderer: Renderer,
}

impl FFXIVRenderer {
    pub async fn new<W: HasRawWindowHandle>(window: &W, width: u32, height: u32) -> Self {
        Self {
            renderer: Renderer::new(window, width, height).await,
        }
    }
    pub fn redraw(&mut self) {
        self.renderer.redraw()
    }
}
