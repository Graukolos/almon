use cgmath::{Angle, Deg, Matrix4, Rad, SquareMatrix, Vector3};

pub struct TransformComponent {
    transform: Matrix4<f32>
}

impl TransformComponent {
    pub fn new() -> TransformComponent {
        let transform = Matrix4::identity();

        TransformComponent {
            transform
        }
    }

    pub fn get_transform(&self) -> [[f32; 4]; 4] {
        self.transform.into()
    }

    pub fn translate(&mut self, shift: Vector3<f32>) {
        self.transform = Matrix4::from_translation(shift) * self.transform;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.transform = Matrix4::from_angle_y(Deg::asin(angle)) * self.transform;
    }
}