use cgmath::Matrix4;

pub trait Camera {
    fn get_vp(&self) -> Matrix4<f32>;
}
