use crate::event::Event;
use crate::renderer::Renderer2D;
use crate::resources::ResourceManager;
use crate::scene::{Scene, TestScene};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct MenuScene {
    resource_manager: Rc<RefCell<ResourceManager>>,
    renderer: Rc<RefCell<dyn Renderer2D>>,
    countdown: f32,
}

impl MenuScene {
    pub fn new(
        renderer: Rc<RefCell<dyn Renderer2D>>,
        resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> MenuScene {
        let countdown = 0.0;

        MenuScene {
            resource_manager,
            renderer,
            countdown,
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
        self.renderer.borrow_mut().render_begin();
        self.renderer.borrow_mut().render_end();
    }

    fn handle(&mut self, _event: Event) {}
}
