use crate::renderer::Renderer2D;
use crate::scene::{MenuScene, Scene};
use crate::ui::Window;
use glium::glutin::event::{ElementState, Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Almon {
    window: Window,
    current_scene: Box<dyn Scene>,
}

impl Almon {
    pub fn new() -> Almon {
        let window = Window::new(800, 600);
        let renderer = Rc::new(RefCell::new(Renderer2D::new(window.get_display())));
        let current_scene = Box::new(MenuScene::new(renderer));

        Almon {
            window,
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
                    WindowEvent::Resized(physical_size) => {
                        almon
                            .current_scene
                            .handle(crate::ui::Event::WindowResizedEvent(
                                physical_size.width as u16,
                                physical_size.height as u16,
                            ));
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        almon
                            .current_scene
                            .handle(crate::ui::Event::KeyPressedEvent(
                                input.scancode as u16,
                                input.state == ElementState::Pressed,
                            ))
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
