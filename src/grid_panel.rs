use piston_window::{G2d, Context, Rectangle, rectangle};

use crate::widget::Widget;

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
    fn draw(&self, context: Context, gl: &mut G2d, width: u32, height: u32) {
        let rectangle = Rectangle::new(self.background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width as f64, height as f64);
        rectangle.draw(square, &context.draw_state, context.transform, gl);
    }
}