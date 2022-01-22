#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, x_tex: f32, y_tex: f32) -> Self {
        let position = [x, y, 0.0];
        let tex_coords = [x_tex, y_tex];

        Self {
            position,
            tex_coords,
        }
    }
}
implement_vertex!(Vertex, position, tex_coords);
