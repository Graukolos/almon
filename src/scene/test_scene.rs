use crate::components::{SpriteRenderComponent, TransformComponent};
use crate::event::Event;
use crate::renderer::Renderer2D;
use crate::resources::ResourceManager;
use crate::scene::Scene;
use cgmath::Vector3;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct TestScene {
    renderer: Rc<RefCell<dyn Renderer2D>>,
    quad: (SpriteRenderComponent, TransformComponent),
}

impl TestScene {
    pub fn new(
        renderer: Rc<RefCell<dyn Renderer2D>>,
        resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> TestScene {
        let mut quad = (
            SpriteRenderComponent::new("quad", "dirt", &resource_manager),
            TransformComponent::new(),
        );
        quad.1.translate(Vector3::new(-0.5, -0.5, 0.0));

        TestScene { renderer, quad }
    }
}

impl Scene for TestScene {
    fn update(&mut self, _dt: &Duration) -> Option<Box<dyn Scene>> {
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().render_begin();
        self.renderer.borrow_mut().draw(&self.quad);
        self.renderer.borrow_mut().render_end();
    }

    fn handle(&mut self, event: Event) {
        match event {
            Event::KeyPressedEvent(keycode) => {
                println!("{}", keycode);
                if keycode == 123 {
                    self.quad.1.translate(Vector3::new(-0.01, 0.0, 0.0));
                } else if keycode == 124 {
                    self.quad.1.translate(Vector3::new(0.01, 0.0, 0.0));
                } else if keycode == 125 {
                    self.quad.1.translate(Vector3::new(0.0, -0.01, 0.0));
                } else if keycode == 126 {
                    self.quad.1.translate(Vector3::new(0.0, 0.01, 0.0));
                }
            }
        }
    }
}
