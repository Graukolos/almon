use crate::components::{SpriteRenderComponent, TransformComponent};
use crate::event::Event;
use crate::renderer::{Camera, Renderer2D};
use crate::resources::ResourceManager;
use crate::scene::Scene;
use cgmath::Vector3;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct TestScene {
    renderer: Rc<RefCell<Renderer2D>>,
    quad: (SpriteRenderComponent, TransformComponent),
    camera: Rc<Camera>,
}

impl TestScene {
    pub fn new(
        renderer: Rc<RefCell<Renderer2D>>,
        resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> TestScene {
        let mut quad = (
            SpriteRenderComponent::new("quad", "dirt", &resource_manager),
            TransformComponent::new(),
        );
        quad.1.translate(Vector3::new(0.0, 0.0, -0.5));
        quad.1.translate(Vector3::new(-0.5, -0.5, 0.0));
        let camera = Rc::new(Camera::new(-2.0, 2.0, -2.0, 2.0));

        TestScene {
            renderer,
            quad,
            camera,
        }
    }
}

impl Scene for TestScene {
    fn update(&mut self, _dt: &Duration) -> Option<Box<dyn Scene>> {
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().begin_render(self.camera.clone());
        self.renderer
            .borrow_mut()
            .draw_quad(self.quad.1.transform(), "dirt");
        self.renderer.borrow_mut().end_render();
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
