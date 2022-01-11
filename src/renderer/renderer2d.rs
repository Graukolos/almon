use crate::physics::TransformComponent;
use crate::renderer::{RenderComponent, Vertex};

pub trait Renderer2D {
    fn render_begin(&mut self);
    fn render_end(&mut self);
    fn draw(&mut self, render_object: &(RenderComponent, TransformComponent));

    fn create_render_component(
        &self,
        mesh: (Vec<Vertex>, Vec<u16>),
        texture: &str,
    ) -> RenderComponent;
}
