use glium::VertexBuffer;

pub struct RenderComponent {
    pub vertex_buffer: VertexBuffer<Vertex>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new2d(x: f32, y: f32) -> Vertex {
        Vertex {
            position: [x, y, 0.0]
        }
    }

    pub fn new3d(x: f32, y:f32, z:f32) -> Vertex {
        Vertex {
            position: [x, y, z]
        }
    }
}
implement_vertex!(Vertex, position);
