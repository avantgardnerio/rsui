use piston_window::*;
use piston_window::text::Text;

use crate::widget::Widget;
use crate::widget::WidgetImpl;

pub struct GridPanel {
    background_color: [f32; 4],
    widget: WidgetImpl
}

impl GridPanel {
    pub fn new(background_color: [f32; 4]) -> Self {
        let widget = WidgetImpl::new();
        GridPanel {
            background_color,
            widget
        }
    }
}

impl Widget for GridPanel {
    fn add_child(&mut self, child: Box<Widget>) {
        self.widget.add_child(child);
    }
    
    fn draw(&self, c: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs) {
        let grid_size = 25;
        let rectangle = Rectangle::new(self.background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width as f64, height as f64);
        rectangle.draw(square, &c.draw_state, c.transform.clone(), gl);

        let white = [1.0, 1.0, 1.0, 1.0];
        for x in (0..(width as i32)).step_by(grid_size) {
            if x % 100 == 0 {
                let transform = c.transform.trans(x as f64 + 2.0, 21.0);
                Text::new_color(white, 24).draw(&x.to_string(), glyphs, &c.draw_state, transform, gl)
                    .unwrap();
            }
            line(white, 1.0, [x as f64, 0.0, x as f64, height], c.transform.clone(), gl);
        }
        for y in (0..(height as i32)).step_by(grid_size) {
            line(white, 1.0, [0.0, y as f64, width, y as f64], c.transform.clone(), gl);
            let transform = c.transform.trans(0.0, y as f64 - 4.0);
            Text::new_color(white, 24).draw(&y.to_string(), glyphs, &c.draw_state, transform, gl)
                .unwrap();
        }
    }
}