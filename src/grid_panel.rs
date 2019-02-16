use piston_window::*;
use piston_window::text::Text;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::widget::Rect;

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
    fn layout(&mut self, bounds: Rect) {
        self.widget.layout(bounds);
        self.widget.children.iter_mut().for_each(|child| {
            let origin = [
                bounds.size[0] / 4.0 + bounds.origin[0],
                bounds.size[1] / 4.0 + bounds.origin[1]
            ];
            let size = [bounds.size[1] / 2.0, bounds.size[1] / 2.0];
            let rect = Rect { origin, size };
            child.layout(rect)
        });
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.widget.add_child(child);
    }
    
    fn draw(&self, c: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs) {
        let w = self.widget.bounds.size[0];
        let h = self.widget.bounds.size[1];
        let left = self.widget.bounds.origin[0];
        let top = self.widget.bounds.origin[1];
        let grid_size = 25;
        let rectangle = Rectangle::new(self.background_color);
        let square = rectangle::rectangle_by_corners(left, top, w, h);
        rectangle.draw(square, &c.draw_state, c.transform.clone(), gl);

        let white = [1.0, 1.0, 1.0, 1.0];
        println!("------- {:?}", self.widget.bounds.size[0]);
        for x in (0..(w as i32)).step_by(grid_size) {
            if x % 100 == 0 {
                let transform = c.transform.trans((x as f64) + 2.0 + left, 21.0 + top);
                Text::new_color(white, 24)
                    .draw(&x.to_string(), glyphs, &c.draw_state, transform, gl)
                    .unwrap();
            }
            let points = [
                (x as f64) + left,
                top,
                (x as f64) + left,
                top + h
            ];
            line(white, 1.0, points, c.transform.clone(), gl);
        }
        for y in (0..(h as i32)).step_by(grid_size) {
            let points = [
                left,
                (y as f64) + top,
                left + w,
                (y as f64) + top
            ];
            line(white, 1.0, points, c.transform.clone(), gl);
            let transform = c.transform.trans(left, (y as f64) - 4.0 + top);
            Text::new_color(white, 24)
                .draw(&y.to_string(), glyphs, &c.draw_state, transform, gl)
                .unwrap();
        }

        self.widget.draw(c, gl, width, height, glyphs)
    }
}