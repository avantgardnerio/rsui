extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics };
use graphics::Context;

pub trait Widget {
    fn draw(&self, context: Context, gl: GlGraphics);
}

