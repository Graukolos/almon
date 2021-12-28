use crate::renderer::{create_program, Renderer2D, RenderComponent, Vertex};
use glium::{Display, Frame, Program, Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::EmptyUniforms;

pub struct SequentialRenderer {
    display: Display,
    test_program: Program,
    current_frame: Option<Frame>
}

impl SequentialRenderer {
    pub fn new(display: Display) -> SequentialRenderer {
        let test_program = create_program(&display, "assets/shaders/test.vert", "assets/shaders/test.frag");

        SequentialRenderer {
            display,
            test_program,
            current_frame: None
        }
    }
}

impl Renderer2D for SequentialRenderer {
    fn render_begin(&mut self) {
        self.current_frame.get_or_insert(self.display.draw());
    }

    fn render_end(&mut self) {
        let frame = self.current_frame.take().unwrap();
        frame.finish().unwrap();
    }

    fn draw(&mut self, render_component: &RenderComponent) {
        let mut frame = self.current_frame.take().unwrap();
        frame.clear_color(0.2, 0.5, 0.3, 1.0);
        let indices = NoIndices(PrimitiveType::TrianglesList);
        frame
            .draw(
                &render_component.vertex_buffer,
                &indices,
                &self.test_program,
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        self.current_frame.insert(frame);
    }

    fn create_render_component(&self, mesh: Vec<Vertex>) -> RenderComponent {
        let vertex_buffer = VertexBuffer::new(&self.display, &mesh).unwrap();

        RenderComponent { vertex_buffer }
    }
}