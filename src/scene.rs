mod menu_scene;
mod test_scene;

use std::time::Duration;
pub use menu_scene::MenuScene;
pub use test_scene::TestScene;
use crate::ui::Event;

pub trait Scene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>>;
    fn render(&mut self);
    fn handle(&mut self, event: Event);
}
