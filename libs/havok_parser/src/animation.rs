use alloc::vec::Vec;

use crate::transform::HavokTransform;

pub trait HavokAnimation {
    fn duration(&self) -> f32;
    fn sample(&self, time: f32) -> Vec<HavokTransform>;
}
