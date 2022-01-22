use crate::components::{SpriteRenderComponent, TransformComponent};
use crate::renderer::{Camera, Vertex};
use ahash::AHashMap;
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub struct Renderer2D {
    display: Rc<Display>,
    texture_program: Program,
    textures: AHashMap<String, Rc<SrgbTexture2d>>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u8>,
    current_frame: Option<Frame>,
    camera: Option<Camera>,
}

impl Renderer2D {
    pub fn new(display: Rc<Display>) -> Self {
        let texture_program = Self::load_program(&*display, "texture");
        let textures = AHashMap::new();
        let vertices = [
            Vertex::new(0.25, 0.25, 0.0, 0.0),
            Vertex::new(0.25, 0.75, 0.0, 1.0),
            Vertex::new(0.75, 0.25, 1.0, 0.0),
            Vertex::new(0.75, 0.75, 1.0, 1.0),
        ];
        let vertex_buffer = VertexBuffer::new(&*display, &vertices).unwrap();
        let indices = [0, 1, 2, 1, 2, 3];
        let index_buffer =
            IndexBuffer::new(&*display, PrimitiveType::TrianglesList, &indices).unwrap();

        Self {
            display,
            texture_program,
            textures,
            vertex_buffer,
            index_buffer,
            current_frame: None,
            camera: None,
        }
    }

    pub fn begin_render(&mut self, camera: Camera) {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        self.current_frame.get_or_insert(frame);
        self.camera.get_or_insert(camera);
    }

    pub fn end_render(&mut self) -> Camera {
        let frame = self.current_frame.take().unwrap();
        frame.finish().unwrap();
        self.camera.take().unwrap()
    }

    pub fn draw_quad(
        &mut self,
        transform_component: &TransformComponent,
        sprite_render_component: &SpriteRenderComponent,
    ) {
        let mut frame = self.current_frame.take().unwrap();
        let tex = self.get_texture(&sprite_render_component.texture);
        let view_projection: [[f32; 4]; 4] = self
            .camera
            .as_ref()
            .unwrap()
            .get_view_projection_matrix()
            .into();
        let model: [[f32; 4]; 4] =
            (transform_component.transform() * sprite_render_component.scale).into();
        let uniforms = uniform! {
            view_projection: view_projection,
            model: model,
            tex: tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.texture_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        self.current_frame.replace(frame);
    }

    fn load_program(display: &Display, program: &str) -> Program {
        let mut vertex_shader_file =
            File::open(format!("assets/shaders/{}.vert", program)).unwrap();
        let mut fragment_shader_file =
            File::open(format!("assets/shaders/{}.frag", program)).unwrap();
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

    fn get_texture(&mut self, texture_name: &str) -> Rc<SrgbTexture2d> {
        match self.textures.get(texture_name) {
            Some(texture) => texture.clone(),
            None => {
                let image = image::open(format!("assets/textures/{}.png", texture_name))
                    .unwrap()
                    .to_rgba8();
                let image_dimensions = image.dimensions();
                let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                let texture = Rc::new(SrgbTexture2d::new(&*self.display, image).unwrap());
                self.textures
                    .insert(String::from(texture_name), texture.clone());
                texture
            }
        }
    }
}
