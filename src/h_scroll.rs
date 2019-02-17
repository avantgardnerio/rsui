use piston_window::*;
use piston_window::text::Text;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::widget::Rect;

pub struct HScroll {
    widget: WidgetImpl
}

impl HScroll {
    pub fn new() -> Self {
        let widget = WidgetImpl::new();
        HScroll {
            widget
        }
    }
}

impl Widget for HScroll {
    fn layout(&mut self, bounds: Rect) {
        println!("HScroll {:?}", bounds);
        self.widget.layout(bounds);
    }

    fn get_bounds(&self) -> Rect {
        return self.widget.bounds;
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.widget.add_child(child);
    }

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        let bounds = self.get_bounds();
        let width = bounds.size[0];
        let height = bounds.size[1];
        let background_color = [0.2, 0.2, 0.2, 1.0];
        let rectangle = Rectangle::new(background_color);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, width, height);
        rectangle.draw(square, &ctx.draw_state, ctx.transform.clone(), gl);

        self.widget.draw(ctx, gl, glyphs)
    }
}