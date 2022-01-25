use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Vector3};

pub struct TransformComponent {
    pub translation: Matrix4<f32>,
    rotation: Matrix4<f32>,
    pub scale: Matrix4<f32>,
}

impl TransformComponent {
    pub fn new(dimensions: (f32, f32)) -> Self {
        let translation = Matrix4::identity();
        let rotation = Matrix4::identity();
        let scale = Matrix4::from_nonuniform_scale(dimensions.0, dimensions.1, 1.0);

        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn transform(&self) -> Matrix4<f32> {
        self.translation * self.rotation
    }

    pub fn translate(&mut self, shift: Vector3<f32>) {
        self.translation = Matrix4::from_translation(shift) * self.translation;
    }

    pub fn rotate(&mut self, angle: Rad<f32>) {
        self.rotation = Matrix4::from_angle_z(angle) * self.rotation;
    }
}
