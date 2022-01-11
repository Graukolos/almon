use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::{Display, Program};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

pub struct ResourceManager {
    display: Rc<Display>,
    shaders: HashMap<String, Rc<Program>>,
    textures: HashMap<String, Rc<SrgbTexture2d>>,
}

impl ResourceManager {
    pub fn new(display: Rc<Display>) -> Self {
        let shaders = HashMap::new();
        let textures = HashMap::new();

        ResourceManager {
            display,
            shaders,
            textures,
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
}
