use glium::{
    Display,
    Surface
};

pub struct Renderer {
    display: Display
}

impl Renderer {
    pub fn new(display: Display) -> Renderer {

        Renderer {
            display
        }
    }

    pub fn draw(&self) {
        let mut target = self.display.draw();
        target.clear_color(0.2, 0.5, 0.3, 1.0);
        target.finish().unwrap();
    }
}