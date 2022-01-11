use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Vector3};

pub struct TransformComponent {
    translation: Matrix4<f32>,
    rotation: Matrix4<f32>,
}

impl TransformComponent {
    pub fn new() -> Self {
        let translation = Matrix4::identity();
        let rotation = Matrix4::identity();

        Self {
            translation,
            rotation,
        }
    }

    pub fn transform(&self) -> [[f32; 4]; 4] {
        (self.translation * self.rotation).into()
    }

    pub fn translate(&mut self, shift: Vector3<f32>) {
        self.translation = Matrix4::from_translation(shift) * self.translation;
    }

    pub fn rotate(&mut self, angle: Rad<f32>) {
        self.rotation = Matrix4::from_angle_z(angle) * self.rotation;
    }
}
