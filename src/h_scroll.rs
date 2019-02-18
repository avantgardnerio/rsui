use piston_window::*;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::widget::Rect;

pub struct HScroll {
    widget: WidgetImpl,
    width: f64,
    min: f64,
    max: f64,
    cur: f64
}

impl HScroll {
    pub fn new() -> Self {
        let widget = WidgetImpl::new();
        HScroll {widget, width: 12.0, min: 0.0, max: 0.0, cur: 0.0}
    }

    pub fn set_min(&mut self, value: f64) {
        self.min = value;
    }

    pub fn set_max(&mut self, value: f64) {
        self.max = value;
    }

    pub fn set_cur(&mut self, value: f64) {
        self.cur = value;
    }

    fn get_slider_size(&self) -> f64 {
        return self.width.max(self.widget.get_bounds().size[0] / (self.max - self.min)); // TODO: step
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

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        let bounds = self.get_bounds();
        let width = bounds.size[0];
        let height = bounds.size[1];

        let background_color = [0.3, 0.3, 0.3, 1.0];
        let rectangle = Rectangle::new(background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        rectangle.draw(square, &ctx.draw_state, ctx.transform.clone(), gl);

        let slider_size = self.get_slider_size();

        let foreground_color = [0.6, 0.6, 0.6, 1.0];
        let rectangle = Rectangle::new(foreground_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, slider_size, height);
        rectangle.draw(square, &ctx.draw_state, ctx.transform.clone(), gl);

        self.widget.draw(ctx, gl, glyphs)
    }
}