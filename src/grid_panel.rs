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
        println!("bounds={:?} color={:?}", bounds, self.background_color);
        self.widget.layout(bounds);
        self.widget.children.iter_mut().for_each(|child| {
            let origin = [bounds.size[0] / 4.0, bounds.size[1] / 4.0];
            let size = [bounds.size[0] / 2.0, bounds.size[1] / 2.0];
            let rect = Rect { origin, size };
            child.layout(rect)
        });
    }

    fn get_bounds(&self) -> Rect {
        return self.widget.bounds;
    }

    fn get_child_bounds(&self) -> Rect {
        return self.widget.get_child_bounds();
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.widget.add_child(child);
    }
    
    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        let width = self.widget.bounds.size[0];
        let height = self.widget.bounds.size[1];
        let grid_size = 25;
        let rectangle = Rectangle::new(self.background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        rectangle.draw(square, &ctx.draw_state, ctx.transform.clone(), gl);

        let white = [1.0, 1.0, 1.0, 1.0];
        for x in (0..(width as i32)).step_by(grid_size) {
            if x % 100 == 0 {
                let transform = ctx.transform.trans((x as f64) + 2.0, 21.0);
                Text::new_color(white, 24)
                    .draw(&x.to_string(), glyphs, &ctx.draw_state, transform, gl)
                    .unwrap();
            }
            let points = [x as f64, 0.0, x as f64, height];
            line(white, 1.0, points, ctx.transform.clone(), gl);
        }
        for y in (0..(height as i32)).step_by(grid_size) {
            let points = [0.0, y as f64, width, y as f64];
            line(white, 1.0, points, ctx.transform.clone(), gl);
            let transform = ctx.transform.trans(2.0, (y as f64) + 21.0);
            Text::new_color(white, 24)
                .draw(&y.to_string(), glyphs, &ctx.draw_state, transform, gl)
                .unwrap();
        }

        self.widget.draw(ctx, gl, glyphs)
    }
}