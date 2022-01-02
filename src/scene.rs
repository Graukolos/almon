use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use crate::renderer::{RenderComponent, Renderer2D, Vertex};
use crate::physics::TransformComponent;

pub trait Scene {
    fn update(&mut self, dt: &Duration);
    fn render(&mut self);
}

pub struct TestScene {
    renderer: Rc<RefCell<dyn Renderer2D>>,
    triangle: (RenderComponent, TransformComponent)
}

impl TestScene {
    pub fn new(renderer: Rc<RefCell<dyn Renderer2D>>) -> TestScene {
        let vertex1 = Vertex::new2d(-0.5, -0.5);
        let vertex2 = Vertex::new2d(0.0, 0.5);
        let vertex3 = Vertex::new2d(0.5, -0.25);
        let mesh = vec![vertex1, vertex2, vertex3];
        let triangle = (renderer.borrow().create_render_component(mesh), TransformComponent::new());

        TestScene { renderer, triangle }
    }
}

impl Scene for TestScene {
    fn update(&mut self, dt: &Duration) {
        self.triangle.1.translate([dt.as_secs_f32()/5.0, 0.0, 0.0]);
        if self.triangle.1.get_transform()[3][0] > 1.5 {
            self.triangle.1.translate([-3.0, 0.0, 0.0]);
        }
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().render_begin();
        self.renderer.borrow_mut().draw(&self.triangle);
        self.renderer.borrow_mut().render_end();
    }
}
