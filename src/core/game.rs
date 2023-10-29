use cgmath::{Deg, InnerSpace, Matrix4, perspective, vec3, Vector3};
use glutin::event_loop::{ControlFlow, EventLoop};
use crate::game_window::GameWindow;
use crate::renderer::Renderer;
use crate::world::World;

pub struct Game { }

impl Game {
    pub fn new() -> Self {
        Game { }
    }

    pub fn run(&self) {
        // Initialize the event loop and window builder
        //the event loop handles events such as keyboard and mouse input, window resizing, and more.
        let event_loop = EventLoop::new();
        let mut window = GameWindow::new();

        // Initialize OpenGL (make opengl functions available within the program)
        gl::load_with(|symbol| window.context.get_proc_address(symbol) as *const _);

        let world = World::new();
        let cube_positions = world.cube_positions.clone();

        let mut renderer = Renderer::new();
        renderer.init_renderer(world);

        // Initialize variables for tracking time
        let mut last_frame_time = std::time::Instant::now();
        let mut delta_time = std::time::Duration::from_secs(0);

        // Main event loop runs until application is terminated.
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            //calculate time between frames
            let current_frame_time = std::time::Instant::now();
            delta_time = current_frame_time.duration_since(last_frame_time);
            last_frame_time = current_frame_time;

            // Convert delta_time to seconds as a floating-point number
            let delta_time = delta_time.as_secs() as f32 + delta_time.subsec_nanos() as f32 / 1_000_000_000.0;

            // events
            window.process_events(event, delta_time, control_flow);

            // update projectin, view, model matrices
            let projection: Matrix4<f32> = perspective(
                Deg(window.camera.zoom),
                16.0 / 9.0,
                0.1,
                100.0
            );

            let view: Matrix4<f32> = window.camera.get_view_matrix();

            let mut models = Vec::new();
            for i in 0..cube_positions.len() {
                models.push(update_model(&cube_positions[i]));
            }

            // render
            renderer.render(projection, view, models);

            window.context.swap_buffers().unwrap();
        });
    }
}

// TODO this shouldn't be here
fn update_model(position : &Vector3<f32>) -> Matrix4<f32> {

    let mut model: Matrix4<f32> = Matrix4::from_translation(*position); //TODO
    let angle = 0.0;
    model * Matrix4::from_axis_angle(
        vec3(1.0, 0.0, 0.0).normalize(),
        Deg(angle)
    )
}