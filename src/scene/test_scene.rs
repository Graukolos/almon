use crate::components::{SpriteRenderComponent, TransformComponent};
use crate::event::Event;
use crate::renderer::{Camera, Renderer2D};
use crate::scene::Scene;
use cgmath::Vector3;
use hecs::World;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

pub struct TestScene {
    renderer: Rc<RefCell<Renderer2D>>,
    camera: Rc<RefCell<Camera>>,
    world: World,
    event_queue: VecDeque<Event>,
}

impl TestScene {
    pub fn new(renderer: Rc<RefCell<Renderer2D>>) -> TestScene {
        let mut world = World::new();
        let mut dirtblock1 = (
            SpriteRenderComponent::new("dirt", (1.0, 1.0)),
            TransformComponent::new(),
        );
        dirtblock1.1.translate(Vector3::new(-0.5, -0.5, -0.5));
        world.spawn(dirtblock1);

        let mut dirtblock2 = (
            SpriteRenderComponent::new("dirt", (1.0, 1.0)),
            TransformComponent::new(),
        );
        dirtblock2.1.translate(Vector3::new(0.5, -0.5, -0.5));
        world.spawn(dirtblock2);

        let camera = Rc::new(RefCell::new(Camera::new(-4.0, 4.0, -3.0, 3.0)));

        TestScene {
            renderer,
            camera,
            world,
            event_queue: VecDeque::new(),
        }
    }

    fn process_events(&mut self) {
        for event in self.event_queue.iter() {
            match event {
                Event::KeyPressedEvent(keycode) => {
                    println!("{}", keycode);
                    if keycode == &123 {
                        //self.quad.1.translate(Vector3::new(-0.01, 0.0, 0.0));
                    } else if keycode == &124 {
                        //self.quad.1.translate(Vector3::new(0.01, 0.0, 0.0));
                    } else if keycode == &125 {
                        //self.quad.1.translate(Vector3::new(0.0, -0.01, 0.0));
                    } else if keycode == &126 {
                        //self.quad.1.translate(Vector3::new(0.0, 0.01, 0.0));
                    }
                }
                Event::WindowResizedEvent(width, height) => {
                    println!("resized");
                    self.camera.borrow_mut().set_projection(
                        -(*width as f32 / 1000.0),
                        *width as f32 / 1000.0,
                        -(*height as f32 / 1000.0),
                        *height as f32 / 1000.0,
                    )
                }
            }
        }
        // Todo: think of a solution for the case where a new event arrives during process_event
        self.event_queue.clear();
    }
}

impl Scene for TestScene {
    fn update(&mut self, _dt: &Duration) -> Option<Box<dyn Scene>> {
        self.process_events();
        // update physics
        None
    }

    fn render(&mut self) {
        self.renderer.borrow_mut().begin_render(self.camera.clone());
        for (_, (render_component, transform_component)) in self
            .world
            .query_mut::<(&SpriteRenderComponent, &TransformComponent)>()
        {
            self.renderer.borrow_mut().draw_quad(
                render_component.scale * transform_component.transform(),
                render_component.texture.as_str(),
            );
        }
        self.renderer.borrow_mut().end_render();
    }

    fn handle(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}
