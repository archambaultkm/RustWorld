use crate::game_specs::*;

use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glutin::dpi::LogicalSize;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use RustWorld::polygon_mode;
use RustWorld::PolygonMode::{Fill, Line};
use RustWorld::camera::{Camera, Camera_Movement::*, Point3};

pub struct GameWindow {
    event_loop : EventLoop<()>,
    pub context : ContextWrapper<PossiblyCurrent, Window>,
    pub camera : Camera,
    last_frame_time: std::time::Instant,
    first_mouse : bool,
    last_x : f32,
    last_y : f32,
}

impl GameWindow {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(TITLE)
            //.with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)); //for set size
            .with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None))); //None for default monitor

        let context = unsafe {
            ContextBuilder::new()
                .build_windowed(window, &event_loop)
                .unwrap()
                .make_current()
        }.unwrap();

        let camera = Camera {
            position: Point3::new(00.0, 10.0, 70.0),
            ..Camera::default()
        };

        let mut first_mouse = true;
        let mut last_x: f32 = WINDOW_WIDTH as f32 / 2.0;
        let mut last_y: f32 = WINDOW_HEIGHT as f32 / 2.0;

        GameWindow {
            event_loop,
            context,
            camera,
            last_frame_time: std::time::Instant::now(),
            first_mouse,
            last_x,
            last_y,
        }
    }
    pub fn process_events(&mut self,
                          event : Event<()>,
                          delta_time : f32,
                          control_flow : &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::KeyboardInput { input, .. } => {
                    self.process_key_input(input, delta_time, control_flow);
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let xpos = position.x as f32;
                    let ypos = position.y as f32;

                    if self.first_mouse {
                        self.last_x = xpos;
                        self.last_y = ypos;
                        self.first_mouse = false;
                    }

                    let xoffset = xpos - self.last_x;
                    let yoffset = self.last_y - ypos; // reversed since y-coordinates go from bottom to top

                    self.last_x = xpos;
                    self.last_y = ypos;

                    self.camera.process_mouse_movement(xoffset, yoffset, true);
                }

                _ => {}
            }

            //This is a catch-all case in the match statement like finally in switch
            _ => (),
        }
    }

    pub fn process_key_input(&mut self, input : KeyboardInput, delta_time : f32, control_flow : &mut ControlFlow) {
        if let Some(key_code) = input.virtual_keycode {
            match key_code {
                VirtualKeyCode::Escape => {
                    if input.state == ElementState::Pressed {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                VirtualKeyCode::W => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(FORWARD, delta_time);
                    }
                }
                VirtualKeyCode::S => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(BACKWARD, delta_time);
                    }
                }
                VirtualKeyCode::A => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(LEFT, delta_time);
                    }
                }
                VirtualKeyCode::D => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(RIGHT, delta_time);
                    }
                }
                VirtualKeyCode::Q => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(UP, delta_time);
                    }
                }
                VirtualKeyCode::Z => {
                    if input.state == ElementState::Pressed {
                        self.camera.process_keyboard(DOWN, delta_time);
                    }
                }
                VirtualKeyCode::F => {
                    if input.state == ElementState::Pressed {
                        polygon_mode(Fill);
                    }
                }
                VirtualKeyCode::L => {
                    if input.state == ElementState::Pressed {
                        polygon_mode(Line);
                    }
                }
                _ => {}
            }
        }
    }
}
