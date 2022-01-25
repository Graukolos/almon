pub struct SpriteRenderComponent {
    pub texture: String,
}

impl SpriteRenderComponent {
    pub fn new(texture_name: &str) -> Self {
        Self {
            texture: texture_name.to_string(),
        }
    }
}
