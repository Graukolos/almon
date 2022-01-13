use crate::renderer::{Camera, Mesh};
use crate::resources::ResourceManager;
use cgmath::Matrix4;
use glium::{Display, Frame, Program, Surface};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Renderer2D {
    display: Rc<Display>,
    _resource_manager: Rc<RefCell<ResourceManager>>,
    texture_program: Rc<Program>,
    quad: Rc<Mesh>,
    current_frame: Option<Frame>,
    camera: Option<Rc<dyn Camera>>,
}

impl Renderer2D {
    pub fn new(display: Rc<Display>, _resource_manager: Rc<RefCell<ResourceManager>>) -> Self {
        let texture_program = _resource_manager.borrow_mut().get_shader("texture");
        let quad = _resource_manager.borrow_mut().get_mesh("quad");

        Self {
            display,
            _resource_manager,
            texture_program,
            quad,
            current_frame: None,
            camera: None,
        }
    }

    pub fn begin_render(&mut self, camera: Rc<dyn Camera>) {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        self.current_frame.get_or_insert(frame);
        self.camera.get_or_insert(camera);
    }

    pub fn end_render(&mut self) {
        let frame = self.current_frame.take().unwrap();
        frame.finish().unwrap();
        self.camera = None;
    }

    pub fn draw_quad(&mut self, transform: Matrix4<f32>, texture: &str) {
        let mut frame = self.current_frame.take().unwrap();
        let tex = self._resource_manager.borrow_mut().get_texture(texture);
        let matrix: [[f32; 4]; 4] = transform.into();
        let uniforms = uniform! {
            matrix: matrix,
            tex: &*tex,
        };
        frame
            .draw(
                &self.quad.vertex_buffer,
                &self.quad.index_buffer,
                &self.texture_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        self.current_frame.replace(frame);
    }
}
