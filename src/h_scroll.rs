use piston_window::*;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::geom::Rect;

pub struct HScroll {
    widget: WidgetImpl,
    width: f64,
    grip_size: f64,
    cur: f64
}

impl HScroll {
    pub fn new() -> Self {
        let widget = WidgetImpl::new();
        HScroll {widget, width: 12.0, grip_size: 0.0, cur: 0.0}
    }

    pub fn set_grip_size(&mut self, value: f64) {
        self.grip_size = value;
    }

    pub fn set_cur(&mut self, value: f64) {
        self.cur = value;
    }
}

impl Widget for HScroll {
    fn layout(&mut self, bounds: Rect) {
        let new_bounds = Rect {
            origin: [0.0, bounds.size[1] - self.width],
            size: [bounds.size[0], self.width]
        };
        self.widget.layout(new_bounds);
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

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs, rect: Rect, depth: i32) {
        let bounds = self.get_bounds();
        let width = bounds.size[0];
        let height = bounds.size[1];

        let track_color = [0.3, 0.3, 0.3, 1.0];
        let track_rect = Rectangle::new(track_color);
        let track_shape = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        track_rect.draw(track_shape, &ctx.draw_state, ctx.transform.clone(), gl);

        let grip_color = [0.6, 0.6, 0.6, 1.0];
        let grip_rect = Rectangle::new(grip_color);
        let grip_shape = rectangle::rectangle_by_corners(0.0, 0.0, self.grip_size, height);
        grip_rect.draw(grip_shape, &ctx.draw_state, ctx.transform.clone(), gl);

        self.widget.draw(ctx, gl, glyphs, rect, depth)
    }
}