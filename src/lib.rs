use gl::types::GLenum;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;

// code found: https://rust-tutorials.github.io/learn-opengl/basics/003-rectangle.html
/// The polygon display modes you can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// Just show the points.
    Point = gl::POINT as isize,
    /// Just show the lines.
    Line = gl::LINE as isize,
    /// Fill in the polygons.
    Fill = gl::FILL as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as GLenum) };
}

