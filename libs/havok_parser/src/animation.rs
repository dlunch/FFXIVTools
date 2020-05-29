use crate::transform::HavokTransform;

pub trait HavokAnimation {
    fn sample(&self, time: f32) -> Vec<HavokTransform>;
}
