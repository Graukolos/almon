use crate::event::Event;
use crate::physics::TransformComponent;
use crate::renderer::{RenderComponent, Renderer2D, Vertex};
use crate::scene::Scene;
use cgmath::Vector3;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct TestScene {
    renderer: Rc<RefCell<dyn Renderer2D>>,
    triangle: (RenderComponent, TransformComponent),
}

impl TestScene {
    pub fn new(renderer: Rc<RefCell<dyn Renderer2D>>) -> TestScene {
        let vertex1 = Vertex::new2d(0.25, 0.25, 0.0, 0.0);
        let vertex2 = Vertex::new2d(0.25, 0.75, 0.0, 1.0);
        let vertex3 = Vertex::new2d(0.75, 0.25, 1.0, 0.0);
        let vertex4 = Vertex::new2d(0.75, 0.75, 1.0, 1.0);
        let mesh = (
            vec![vertex1, vertex2, vertex3, vertex4],
            vec![0, 1, 2, 1, 2, 3],
        );
        let mut triangle = (
            renderer
                .borrow()
                .create_render_component(mesh, "assets/textures/dirt.png"),
            TransformComponent::new(),
        );
        triangle.1.translate(Vector3::new(-0.5, -0.5, 0.0));

        TestScene { renderer, triangle }
    }
}

impl Scene for TestScene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>> {
        //self.triangle.1.translate(Vector3::new(dt.as_secs_f32()/5.0, 0.0, 0.0));
        //if self.triangle.1.get_transform()[3][0] > 1.5 {
        //    self.triangle.1.translate(Vector3::new(-3.0, 0.0, 0.0));
        //}
        //self.triangle.1.rotate(dt.as_secs_f32() * 100.0);
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().render_begin();
        self.renderer.borrow_mut().draw(&self.triangle);
        self.renderer.borrow_mut().render_end();
    }

    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyPressedEvent(keycode) => {
                println!("{}", keycode);
                if keycode == 123 {
                    self.triangle.1.translate(Vector3::new(-0.01, 0.0, 0.0));
                } else if keycode == 124 {
                    self.triangle.1.translate(Vector3::new(0.01, 0.0, 0.0));
                } else if keycode == 125 {
                    self.triangle.1.translate(Vector3::new(0.0, -0.01, 0.0));
                } else if keycode == 126 {
                    self.triangle.1.translate(Vector3::new(0.0, 0.01, 0.0));
                }
            }
        }
    }
}
