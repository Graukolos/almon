use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use crate::renderer::Renderer2D;
use crate::scene::{Scene, TestScene};

pub struct MenuScene {
    renderer: Rc<RefCell<dyn Renderer2D>>,
    countdown: f32
}

impl MenuScene {
    pub fn new(renderer: Rc<RefCell<dyn Renderer2D>>) -> MenuScene {
        let countdown = 0.0;

        MenuScene { renderer, countdown }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>> {
        self.countdown += dt.as_secs_f32();
        if self.countdown > 10.0 {
            return Some(Box::new(TestScene::new(self.renderer.clone())))
        }
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().render_begin();
        self.renderer.borrow_mut().render_end();
    }
}