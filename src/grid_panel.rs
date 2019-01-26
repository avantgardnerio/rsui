use graphics::Context;
use crate::widget::Widget;
use graphics::rectangle;
use opengl_graphics::GlGraphics;

pub struct GridPanel {
    background_color: [f32; 4]
}

impl GridPanel {
    pub fn new(background_color: [f32; 4]) -> Self {
        GridPanel {
            background_color
        }
    }
}

impl Widget for GridPanel {
    fn draw(&self, context: Context, gl: &mut GlGraphics, width: f64, height: f64) {
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        rectangle(self.background_color, square, context.transform, gl);
    }
}