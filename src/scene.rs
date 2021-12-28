use crate::renderer::{RenderComponent, Renderer, Vertex};
use std::rc::Rc;
use std::time::Duration;

pub trait Scene {
    fn update(&mut self, dt: &Duration);
    fn render(&self);
}

pub struct TestScene {
    renderer: Rc<Renderer>,
    triangle: RenderComponent,
}

impl TestScene {
    pub fn new(renderer: Rc<Renderer>) -> TestScene {
        let vertex1 = Vertex::new2d(-0.5, -0.5);
        let vertex2 = Vertex::new2d(0.0, 0.5);
        let vertex3 = Vertex::new2d(0.5, -0.25);
        let mesh = vec![vertex1, vertex2, vertex3];
        let triangle = renderer.create_render_component(mesh);

        TestScene { renderer, triangle }
    }
}

impl Scene for TestScene {
    fn update(&mut self, dt: &Duration) {}

    fn render(&self) {
        self.renderer.draw(&self.triangle);
    }
}
