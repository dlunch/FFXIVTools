use std::cell::RefCell;
use std::sync::Arc;

use crate::{animation::HavokAnimation, object::HavokObject, transform::HavokTransform};

pub struct HavokSplineCompressedAnimation {
    pub duration: f32,
}

impl HavokSplineCompressedAnimation {
    pub fn new(object: Arc<RefCell<HavokObject>>) -> Self {
        let root = object.borrow();

        let duration = root.get("duration").as_real();

        Self { duration }
    }
}

impl HavokAnimation for HavokSplineCompressedAnimation {
    #[allow(unused_variables)]
    fn sample(&self, time: f32) -> Vec<HavokTransform> {
        Vec::new()
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}
