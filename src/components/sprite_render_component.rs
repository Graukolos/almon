use cgmath::Matrix4;

pub struct SpriteRenderComponent {
    pub scale: Matrix4<f32>,
    pub texture: String,
}

impl SpriteRenderComponent {
    pub fn new(texture_name: &str, dimensions: (f32, f32)) -> Self {
        let scale = Matrix4::from_nonuniform_scale(dimensions.0, dimensions.1, 1.0);

        Self {
            scale,
            texture: texture_name.to_string(),
        }
    }
}
