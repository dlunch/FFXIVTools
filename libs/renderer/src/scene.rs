use crate::{Camera, Renderable};

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self { camera, models: Vec::new() }
    }

    pub fn add_model(&mut self, model: Box<dyn Renderable>) {
        self.models.push(model)
    }
}
