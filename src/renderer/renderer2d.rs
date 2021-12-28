use crate::renderer::{RenderComponent, Vertex};
use glium::{Display, Program};
use std::fs::File;
use std::io::Read;

pub trait Renderer2D {
    fn render_begin(&mut self);
    fn render_end(&mut self);
    fn draw(&mut self, render_component: &RenderComponent);

    fn create_render_component(&self, mesh: Vec<Vertex>) -> RenderComponent;
}

pub fn create_program(display: &Display, vertex_shader_path: &str, fragment_shader_path: &str) -> Program {
    let mut vertex_shader_file = File::open(vertex_shader_path).unwrap();
    let mut fragment_shader_file = File::open(fragment_shader_path).unwrap();
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    vertex_shader_file
        .read_to_string(&mut vertex_shader_src)
        .unwrap();
    fragment_shader_file
        .read_to_string(&mut fragment_shader_src)
        .unwrap();

    Program::from_source(
        display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    )
        .unwrap()
}