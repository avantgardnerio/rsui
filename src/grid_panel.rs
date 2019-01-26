use graphics::Context;
use crate::widget::Widget;
use graphics::rectangle;
use opengl_graphics::GlGraphics;
use graphics::*;

pub struct GridPanel {

}

impl GridPanel {

}

impl Widget for GridPanel {
    fn draw(&self, context: Context, gl: &mut GlGraphics, width: f64, height: f64) {
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (width / 2.0 - 25.0, height / 2.0 - 25.0);
        let transform = context.transform.trans(x, y);
        rectangle(RED, square, transform, gl);
    }
}