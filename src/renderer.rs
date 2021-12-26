use glium::{
    uniforms::EmptyUniforms,
    index::{
        NoIndices,
        PrimitiveType
    },
    Display, Program, Surface, VertexBuffer};
use std::{fs::File, io::Read};

pub struct Renderer {
    display: Display,
    test_program: Program,
}

impl Renderer {
    pub fn new(display: Display) -> Renderer {
        let test_program = Renderer::create_program(
            &display,
            "assets/shaders/test.vert",
            "assets/shaders/test.frag",
        );

        Renderer {
            display,
            test_program,
        }
    }

    pub fn draw(&self, render_component: &RenderComponent) {
        let mut target = self.display.draw();
        target.clear_color(0.2, 0.5, 0.3, 1.0);
        let indices = NoIndices(PrimitiveType::TrianglesList);
        target
            .draw(
                &render_component.vertex_buffer,
                &indices,
                &self.test_program,
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    pub fn create_render_component(&self, mesh: std::vec::Vec<Vertex>) -> RenderComponent {
        let vertex_buffer = VertexBuffer::new(&self.display, &mesh).unwrap();

        RenderComponent { vertex_buffer }
    }

    fn create_program(
        display: &Display,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Program {
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
}

pub struct RenderComponent {
    vertex_buffer: VertexBuffer<Vertex>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);
