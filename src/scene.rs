use std::time::Duration;
use crate::renderer::Renderer;

pub trait Scene {
    fn update(&mut self, dt: &Duration);
    fn render(&self, renderer: &Renderer);
}

pub struct TestScene {

}

impl TestScene {
    pub fn new() -> TestScene {

        TestScene {

        }
    }
}

impl Scene for TestScene {
    fn update(&mut self, dt: &Duration) {
        println!("test scene update");
    }

    fn render(&self, renderer: &Renderer) {
        renderer.draw();
    }
}