use crate::renderer::{Camera, Renderer2D};
use crate::scene::{Scene, TestScene};
use crate::ui::Event;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct MenuScene {
    renderer: Rc<RefCell<Renderer2D>>,
    countdown: f32,
    camera: Option<Camera>,
}

impl MenuScene {
    pub fn new(renderer: Rc<RefCell<Renderer2D>>) -> MenuScene {
        let countdown = 0.0;
        let camera = Some(Camera::new(-2.0, 2.0, -2.0, 2.0));

        MenuScene {
            renderer,
            countdown,
            camera,
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>> {
        self.countdown += dt.as_secs_f32();
        if self.countdown > 3.0 {
            return Some(Box::new(TestScene::new(self.renderer.clone())));
        }
        None
    }

    fn render(&mut self) {
        self.renderer
            .borrow_mut()
            .begin_render(self.camera.take().unwrap());
        self.camera = Some(self.renderer.borrow_mut().end_render());
    }

    fn handle(&mut self, _event: Event) {}
}
