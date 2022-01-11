use crate::physics::TransformComponent;
use crate::renderer::{RenderComponent, Renderer2D, Vertex};
use crate::resources::ResourceManager;
use glium::index::PrimitiveType;
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SequentialRenderer {
    display: Rc<Display>,
    resource_manager: Rc<RefCell<ResourceManager>>,
    texture_program: Rc<Program>,
    current_frame: Option<Frame>,
}

impl SequentialRenderer {
    pub fn new(
        display: Rc<Display>,
        resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> SequentialRenderer {
        let texture_program = resource_manager.borrow_mut().get_shader("texture");

        SequentialRenderer {
            display,
            resource_manager,
            texture_program,
            current_frame: None,
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
            tex: &*render_component.texture
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

    fn create_render_component(
        &self,
        mesh: (Vec<Vertex>, Vec<u16>),
        texture: &str,
    ) -> RenderComponent {
        let (vertices, indices) = mesh;
        let vertex_buffer = VertexBuffer::new(&*self.display, &vertices).unwrap();
        let index_buffer =
            IndexBuffer::new(&*self.display, PrimitiveType::TrianglesList, &indices).unwrap();
        let texture = self.resource_manager.borrow_mut().get_texture(texture);

        RenderComponent {
            vertex_buffer,
            index_buffer,
            texture,
        }
    }
}
