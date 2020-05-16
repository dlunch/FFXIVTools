use crate::{Camera, Renderable};

pub struct Scene<'a> {
    pub camera: Camera,
    pub models: Vec<Box<dyn Renderable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera) -> Self {
        Self { camera, models: Vec::new() }
    }

    pub fn add<F: Renderable + 'a>(&mut self, model: F) -> &mut Box<F> {
        self.models.push(Box::new(model));

        let len = self.models.len();
        unsafe { std::mem::transmute(&mut self.models[len - 1]) }
    }
}
