use crate::components::{SpriteRenderComponent, TransformComponent};
use crate::renderer::Renderer2D;
use crate::resources::ResourceManager;
use glium::{Display, Frame, Program, Surface};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SequentialRenderer {
    display: Rc<Display>,
    _resource_manager: Rc<RefCell<ResourceManager>>,
    texture_program: Rc<Program>,
    current_frame: Option<Frame>,
}

impl SequentialRenderer {
    pub fn new(
        display: Rc<Display>,
        _resource_manager: Rc<RefCell<ResourceManager>>,
    ) -> SequentialRenderer {
        let texture_program = _resource_manager.borrow_mut().get_shader("texture");

        SequentialRenderer {
            display,
            _resource_manager,
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

    fn draw(&mut self, render_object: &(SpriteRenderComponent, TransformComponent)) {
        let (render_component, transform_component) = render_object;
        let mut frame = self.current_frame.take().unwrap();
        let uniforms = uniform! {
            matrix: transform_component.transform(),
            tex: &*render_component.texture
        };
        frame
            .draw(
                &render_component.mesh.vertex_buffer,
                &render_component.mesh.index_buffer,
                &self.texture_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        self.current_frame.replace(frame);
    }
}
