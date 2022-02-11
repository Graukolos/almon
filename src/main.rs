#[macro_use]
extern crate glium;

mod almon;
mod components;
mod renderer;
mod scene;
mod ui;

use almon::Almon;

fn main() {
    Almon::new().run();
}
