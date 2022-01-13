use crate::event::Event;
use crate::renderer::{Camera, OrthographicCamera, Renderer2D};
use crate::resources::ResourceManager;
use crate::scene::{Scene, TestScene};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct MenuScene {
    resource_manager: Rc<RefCell<ResourceManager>>,
    renderer: Rc<RefCell<Renderer2D>>,
    countdown: f32,
    camera: Rc<dyn Camera>,
}

impl MenuScene {
    pub fn new(
        renderer: Rc<RefCell<Renderer2D>>,
        resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> MenuScene {
        let countdown = 0.0;
        let camera = Rc::new(OrthographicCamera::new(-1.0, 1.0, -1.0, 1.0));

        MenuScene {
            resource_manager,
            renderer,
            countdown,
            camera,
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>> {
        self.countdown += dt.as_secs_f32();
        if self.countdown > 5.0 {
            return Some(Box::new(TestScene::new(
                self.renderer.clone(),
                self.resource_manager.clone(),
            )));
        }
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().begin_render(self.camera.clone());
        self.renderer.borrow_mut().end_render();
    }

    fn handle(&mut self, _event: Event) {}
}
