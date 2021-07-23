use nalgebra::Point3;
use winit::window::Window;

use common::{regions, WasmPackage};
use renderer::{Camera, Renderer, Scene, WindowRenderTarget};

#[allow(dead_code)]
pub struct Content {
    renderer: Renderer,
    window: Window,
    render_target: WindowRenderTarget,
    package: WasmPackage,
    scene: Scene,
}

impl Content {
    pub async fn new(window: Window) -> Self {
        let package = WasmPackage::new(&regions()[0], "https://ffxiv-data.dlunch.net/compressed").await;

        let size = window.inner_size();
        let renderer = Renderer::new().await;
        let render_target = WindowRenderTarget::new(&renderer, &window, size.width, size.height);

        let camera = Camera::new(Point3::new(0.0, 0.8, 2.5), Point3::new(0.0, 0.8, 0.0));
        let scene = Scene::new(camera);

        Self {
            renderer,
            window,
            render_target,
            package,
            scene,
        }
    }

    pub fn redraw(&mut self) {
        self.renderer.render(&self.scene, &mut self.render_target);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
