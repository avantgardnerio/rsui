use piston_window::{G2d, Context, Rectangle, rectangle, line};

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
    fn draw(&self, context: Context, gl: &mut G2d, width: f64, height: f64) {
        let rectangle = Rectangle::new(self.background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width as f64, height as f64);
        rectangle.draw(square, &context.draw_state, context.transform.clone(), gl);

        let white = [1.0, 1.0, 1.0, 1.0];
        for x in (0..(width as i32)).step_by(100) {
            line(white, 1.0, [x as f64, 0.0, x as f64, height], context.transform.clone(), gl);
        }
        for y in (0..(height as i32)).step_by(100) {
            line(white, 1.0, [0.0, y as f64, width, y as f64], context.transform.clone(), gl);
        }
    }
}