extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use graphics::Context;
use opengl_graphics::GlGraphics;

pub trait Widget {
    fn draw(&self, context: Context, gl: &mut GlGraphics, width: f64, height: f64);
}

