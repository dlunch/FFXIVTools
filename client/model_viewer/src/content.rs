use log::debug;
use winit::{event_loop::EventLoop, window::Window, window::WindowBuilder};

pub struct Content {
    window: Window,
}

impl Content {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        #[allow(unused_mut)]
        let mut builder = WindowBuilder::new();

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::HtmlCanvasElement;
            use winit::platform::web::WindowBuilderExtWebSys;

            builder = builder.with_canvas(self.canvas.cast::<HtmlCanvasElement>());
        }

        let window = builder.build(event_loop).unwrap();

        Self { window }
    }

    pub fn redraw(&self) {
        debug!("redraw")
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
