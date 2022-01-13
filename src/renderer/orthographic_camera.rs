use crate::renderer::Camera;
use cgmath::prelude::*;
use cgmath::{Matrix4, Perspective, Vector3};

pub struct OrthographicCamera {
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let view_matrix = Matrix4::identity();
        let projection_matrix = Matrix4::from(Perspective {
            left,
            right,
            bottom,
            top,
            near: -1.0,
            far: 1.0,
        });
        let view_projection_matrix = projection_matrix * view_matrix;

        Self {
            view_matrix,
            projection_matrix,
            view_projection_matrix,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.view_matrix = Matrix4::from_translation(position).invert().unwrap();
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn set_projection(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        self.projection_matrix = Matrix4::from(Perspective {
            left,
            right,
            bottom,
            top,
            near: -1.0,
            far: 1.0,
        });
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }
}

impl Camera for OrthographicCamera {
    fn get_vp(&self) -> Matrix4<f32> {
        self.view_projection_matrix
    }
}
