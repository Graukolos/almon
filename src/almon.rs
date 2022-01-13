use crate::renderer::Renderer2D;
use crate::resources::ResourceManager;
use crate::scene::{MenuScene, Scene};
use crate::window::Window;
use glium::glutin::event::{ElementState, Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Almon {
    window: Window,
    _resource_manager: Rc<RefCell<ResourceManager>>,
    _renderer: Rc<RefCell<Renderer2D>>,
    current_scene: Box<dyn Scene>,
}

impl Almon {
    pub fn new() -> Almon {
        let window = Window::default();
        let _resource_manager = Rc::new(RefCell::new(ResourceManager::new(window.get_display())));
        let _renderer = Rc::new(RefCell::new(Renderer2D::new(
            window.get_display(),
            _resource_manager.clone(),
        )));
        let current_scene = Box::new(MenuScene::new(_renderer.clone(), _resource_manager.clone()));

        Almon {
            window,
            _resource_manager,
            _renderer,
            current_scene,
        }
    }

    pub fn run(mut almon: Almon) {
        let tickrate = 128;
        let dt = Duration::from_millis(1000 / tickrate);
        let mut accumulator = Duration::from_millis(0);
        let mut frame_start = Instant::now();
        let event_loop = almon.window.event_loop.take().unwrap();

        event_loop.run(move |ev, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Pressed {
                            almon
                                .current_scene
                                .handle(crate::event::Event::KeyPressedEvent(input.scancode as u16))
                        }
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    accumulator += frame_start.elapsed();
                    frame_start = Instant::now();

                    if accumulator > Duration::from_millis(200) {
                        accumulator = Duration::from_millis(200);
                    }

                    while accumulator > dt {
                        if let Some(scene) = almon.current_scene.update(&dt) {
                            almon.current_scene = scene;
                        }
                        accumulator -= dt;
                    }

                    almon.current_scene.render();
                }
                _ => {}
            }
        });
    }
}
