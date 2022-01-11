use crate::renderer::{Mesh, Vertex};
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{Display, IndexBuffer, Program, VertexBuffer};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub struct ResourceManager {
    display: Rc<Display>,
    shaders: HashMap<String, Rc<Program>>,
    textures: HashMap<String, Rc<SrgbTexture2d>>,
    meshes: HashMap<String, Rc<Mesh>>,
}

impl ResourceManager {
    pub fn new(display: Rc<Display>) -> Self {
        let shaders = HashMap::new();
        let textures = HashMap::new();
        let meshes = HashMap::new();

        ResourceManager {
            display,
            shaders,
            textures,
            meshes,
        }
    }

    pub fn get_texture(&mut self, texture_name: &str) -> Rc<SrgbTexture2d> {
        match self.textures.get(texture_name) {
            Some(texture) => texture.clone(),
            None => {
                let texture = Rc::new(self.load_texture(texture_name));
                self.textures
                    .insert(String::from(texture_name), texture.clone());
                texture
            }
        }
    }

    pub fn get_shader(&mut self, shader_name: &str) -> Rc<Program> {
        match self.shaders.get(shader_name) {
            Some(shader) => shader.clone(),
            None => {
                let shader = Rc::new(self.load_shader(shader_name));
                self.shaders
                    .insert(String::from(shader_name), shader.clone());
                shader
            }
        }
    }

    pub fn get_mesh(&mut self, mesh_name: &str) -> Rc<Mesh> {
        match self.meshes.get(mesh_name) {
            Some(mesh) => mesh.clone(),
            None => {
                let mesh = Rc::new(self.load_mesh(mesh_name));
                self.meshes.insert(String::from(mesh_name), mesh.clone());
                mesh
            }
        }
    }

    fn load_texture(&self, texture_name: &str) -> SrgbTexture2d {
        let image = image::open(format!("assets/textures/{}.png", texture_name))
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&*self.display, image).unwrap();
        texture
    }

    fn load_shader(&self, shader_name: &str) -> Program {
        let mut vertex_shader_file =
            File::open(format!("assets/shaders/{}.vert", shader_name)).unwrap();
        let mut fragment_shader_file =
            File::open(format!("assets/shaders/{}.frag", shader_name)).unwrap();
        let mut vertex_shader_src = String::new();
        let mut fragment_shader_src = String::new();
        vertex_shader_file
            .read_to_string(&mut vertex_shader_src)
            .unwrap();
        fragment_shader_file
            .read_to_string(&mut fragment_shader_src)
            .unwrap();

        Program::from_source(
            &*self.display,
            vertex_shader_src.as_str(),
            fragment_shader_src.as_str(),
            None,
        )
        .unwrap()
    }

    fn load_mesh(&self, _mesh_name: &str) -> Mesh {
        let vertices = vec![
            Vertex::new(0.25, 0.25, 0.0, 0.0),
            Vertex::new(0.25, 0.75, 0.0, 1.0),
            Vertex::new(0.75, 0.25, 1.0, 0.0),
            Vertex::new(0.75, 0.75, 1.0, 1.0),
        ];
        let indices = vec![0, 1, 2, 1, 2, 3];

        let vertex_buffer = VertexBuffer::new(&*self.display, &vertices).unwrap();
        let index_buffer =
            IndexBuffer::new(&*self.display, PrimitiveType::TrianglesList, &indices).unwrap();

        Mesh {
            vertex_buffer,
            index_buffer,
        }
    }
}
