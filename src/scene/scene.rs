use std::time::Duration;

pub trait Scene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>>;
    fn render(&mut self);
}