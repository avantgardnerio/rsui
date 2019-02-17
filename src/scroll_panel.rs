use piston_window::*;
use piston_window::text::Text;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::widget::Rect;
use crate::h_scroll::HScroll;

pub struct ScrollPanel {
    widget: WidgetImpl
}

impl ScrollPanel {
    pub fn new() -> Self {
        let mut widget = WidgetImpl::new();
        let h_scroll = Box::new(HScroll::new());
        widget.add_child(h_scroll);
        ScrollPanel {
            widget
        }
    }
}

impl Widget for ScrollPanel {
    fn layout(&mut self, bounds: Rect) {
        println!("ScrollPanel {:?}", bounds);
        self.widget.layout(bounds);
    }

    fn get_bounds(&self) -> Rect {
        return self.widget.bounds;
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.widget.children.insert(0, child);
    }

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        self.widget.draw(ctx, gl, glyphs)
    }
}