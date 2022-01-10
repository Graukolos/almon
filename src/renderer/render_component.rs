use glium::texture::SrgbTexture2d;
use glium::{IndexBuffer, VertexBuffer};

pub struct RenderComponent {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub texture: SrgbTexture2d,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new2d(x: f32, y: f32, x_tex: f32, y_tex: f32) -> Vertex {
        Vertex {
            position: [x, y, 0.0],
            tex_coords: [x_tex, y_tex],
        }
    }

    pub fn new3d(x: f32, y: f32, z: f32, x_tex: f32, y_tex: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
            tex_coords: [x_tex, y_tex],
        }
    }
}
implement_vertex!(Vertex, position, tex_coords);
