use crate::renderer::Mesh;
use crate::resources::ResourceManager;
use glium::texture::SrgbTexture2d;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SpriteRenderComponent {
    pub mesh: Rc<Mesh>,
    pub texture: Rc<SrgbTexture2d>,
}

impl SpriteRenderComponent {
    pub fn new(
        _mesh_name: &str,
        _texture_name: &str,
        resource_manager: &Rc<RefCell<ResourceManager>>,
    ) -> Self {
        let mesh = resource_manager.borrow_mut().get_mesh("quad");
        let texture = resource_manager.borrow_mut().get_texture("dirt");

        Self { mesh, texture }
    }
}
