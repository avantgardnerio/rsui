use piston_window::{G2d, Context, Glyphs};

pub trait Widget {
    fn draw(&self, context: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs);
}

