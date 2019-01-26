use graphics::Context;
use crate::widget::Widget;
use graphics::rectangle;
use opengl_graphics::GlGraphics;
use graphics::*;

pub struct GridPanel {
    backgroundColor: [f32; 4]
}

impl GridPanel {
    pub fn new(backgroundColor: [f32; 4]) -> Self {
        GridPanel {
            backgroundColor
        }
    }
}

impl Widget for GridPanel {
    fn draw(&self, context: Context, gl: &mut GlGraphics, width: f64, height: f64) {
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        rectangle(self.backgroundColor, square, context.transform, gl);
    }
}