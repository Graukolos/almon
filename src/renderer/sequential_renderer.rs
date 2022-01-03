use crate::renderer::{create_program, Renderer2D, RenderComponent, Vertex};
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::RawImage2d;
use crate::physics::TransformComponent;

pub struct SequentialRenderer {
    display: Display,
    texture_program: Program,
    current_frame: Option<Frame>
}

impl SequentialRenderer {
    pub fn new(display: Display) -> SequentialRenderer {
        let texture_program = create_program(&display, "assets/shaders/texture.vert", "assets/shaders/texture.frag");

        SequentialRenderer {
            display,
            texture_program,
            current_frame: None
        }
    }
}

impl Renderer2D for SequentialRenderer {
    fn render_begin(&mut self) {
        let mut frame = self.display.draw();
        frame.clear_color(0.2, 0.5, 0.3, 1.0);
        self.current_frame.get_or_insert(frame);

    }

    fn render_end(&mut self) {
        let frame = self.current_frame.take().unwrap();
        frame.finish().unwrap();
    }

    fn draw(&mut self, render_object: &(RenderComponent, TransformComponent)) {
        let (render_component, transform_component) = render_object;
        let mut frame = self.current_frame.take().unwrap();
        let uniforms = uniform! {
            matrix: transform_component.get_transform(),
            tex: &render_component.texture
        };
        frame
            .draw(
                &render_component.vertex_buffer,
                &render_component.index_buffer,
                &self.texture_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        self.current_frame.replace(frame);
    }

    fn create_render_component(&self, mesh: (Vec<Vertex>, Vec<u16>), texture: &str) -> RenderComponent {
        let (vertices, indices) = mesh;
        let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();
        let index_buffer = IndexBuffer::new(&self.display, PrimitiveType::TrianglesList, &indices).unwrap();
        let image = image::open(texture).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();

        RenderComponent {
            vertex_buffer,
            index_buffer,
            texture
        }
    }
}