#[macro_use]
extern crate glium;
extern crate ahash;
extern crate cgmath;
extern crate hecs;
extern crate image;

mod almon;
mod components;
mod renderer;
mod scene;
mod ui;

use almon::Almon;

fn main() {
    Almon::run(Almon::new());
}
