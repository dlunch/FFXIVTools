use nalgebra::{Matrix4, Point3};

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
}

impl Camera {
    pub fn new(eye: Point3<f32>, target: Point3<f32>) -> Self {
        Camera { eye, target }
    }

    pub fn view(&self) -> Matrix4<f32> {
        nalgebra::Matrix4::look_at_rh(&self.eye, &self.target, &nalgebra::Vector3::y_axis())
    }
}
