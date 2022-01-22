use crate::components::{PlayerControllerComponent, SpriteRenderComponent, TransformComponent};
use crate::renderer::{Camera, Renderer2D};
use crate::scene::Scene;
use crate::ui::Event;
use cgmath::Vector3;
use hecs::World;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

pub struct TestScene {
    renderer: Rc<RefCell<Renderer2D>>,
    camera: Option<Camera>,
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

        let player = (
            SpriteRenderComponent::new("player", (1.0, 2.0)),
            TransformComponent::new(),
            PlayerControllerComponent::new(),
        );
        world.spawn(player);

        let camera = Some(Camera::new(-4.0, 4.0, -3.0, 3.0));

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
                Event::KeyPressedEvent(keycode, state) => {
                    if keycode == &123 {
                        for (_, player_controller_component) in
                            self.world.query_mut::<&mut PlayerControllerComponent>()
                        {
                            player_controller_component.set_moving_left(*state);
                        }
                    } else if keycode == &124 {
                        for (_, player_controller_component) in
                            self.world.query_mut::<&mut PlayerControllerComponent>()
                        {
                            player_controller_component.set_moving_right(*state);
                        }
                    } else if keycode == &125 {
                        for (_, player_controller_component) in
                            self.world.query_mut::<&mut PlayerControllerComponent>()
                        {
                            player_controller_component.set_moving_down(*state);
                        }
                    } else if keycode == &126 {
                        for (_, player_controller_component) in
                            self.world.query_mut::<&mut PlayerControllerComponent>()
                        {
                            player_controller_component.set_moving_up(*state);
                        }
                    }
                }
                Event::WindowResizedEvent(width, height) => {
                    self.camera.as_mut().unwrap().set_projection(
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

    fn update_physics(&mut self, dt: &Duration) {
        for (_, (player_controller_component, transform_component)) in self
            .world
            .query_mut::<(&mut PlayerControllerComponent, &mut TransformComponent)>()
        {
            if player_controller_component.moving_left {
                transform_component.translate(Vector3::new(-0.1, 0.0, 0.0) * dt.as_secs_f32())
            }
            if player_controller_component.moving_right {
                transform_component.translate(Vector3::new(0.1, 0.0, 0.0) * dt.as_secs_f32())
            }
            if player_controller_component.moving_up {
                transform_component.translate(Vector3::new(0.0, 0.1, 0.0) * dt.as_secs_f32())
            }
            if player_controller_component.moving_down {
                transform_component.translate(Vector3::new(0.0, -0.1, 0.0) * dt.as_secs_f32())
            }
            self.camera
                .as_mut()
                .unwrap()
                .set_position(transform_component.position())
        }
    }
}

impl Scene for TestScene {
    fn update(&mut self, dt: &Duration) -> Option<Box<dyn Scene>> {
        self.process_events();
        self.update_physics(dt);
        None
    }

    fn render(&mut self) {
        self.renderer
            .borrow_mut()
            .begin_render(self.camera.take().unwrap());
        for (_, (render_component, transform_component)) in self
            .world
            .query_mut::<(&SpriteRenderComponent, &TransformComponent)>()
        {
            self.renderer
                .borrow_mut()
                .draw_quad(transform_component, render_component);
        }
        self.camera = Some(self.renderer.borrow_mut().end_render());
    }

    fn handle(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}
