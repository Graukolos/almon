use std::time::Duration;

pub trait Scene {
    fn update(&mut self, dt: &Duration);
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
}