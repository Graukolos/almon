use crate::components::{SpriteRenderComponent, TransformComponent};

pub trait Renderer2D {
    fn render_begin(&mut self);
    fn render_end(&mut self);
    fn draw(&mut self, render_object: &(SpriteRenderComponent, TransformComponent));
}
