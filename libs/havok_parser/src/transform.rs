use nalgebra::{Quaternion, Vector4};

use crate::object::HavokReal;

pub struct HavokTransform {
    pub translation: Vector4<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector4<f32>,
}

impl HavokTransform {
    pub fn new(vec: &Vec<HavokReal>) -> Self {
        Self {
            translation: Vector4::new(vec[0], vec[1], vec[2], vec[3]),
            rotation: Quaternion::new(vec[4], vec[5], vec[6], vec[7]),
            scale: Vector4::new(vec[8], vec[9], vec[10], vec[11]),
        }
    }
}
