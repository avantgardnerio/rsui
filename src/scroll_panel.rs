use piston_window::*;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::geom::Rect;
use crate::h_scroll::HScroll;
use crate::geom::Point;

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
        self.widget.bounds = bounds;
        self.widget.children.iter_mut().for_each(|child| {
            child.downcast_mut::<HScroll>().map(|h_scroll| {
                h_scroll.layout(bounds);
            });
        });

        // http://csdgn.org/article/scrollbar
        let window_size: f64 = self.get_bounds().size[0];
        let track_size = window_size; // Same for now
        let content_size: f64 = self.get_child_bounds().size[0];
        let window_content_ratio = window_size / content_size;
        let grip_size = track_size * window_content_ratio;
        self.widget.children.iter_mut().for_each(|child| {
            child.downcast_mut::<HScroll>().map(|h_scroll| {
                h_scroll.layout(bounds);
                h_scroll.set_grip_size(grip_size);
            });
        });
    }

    fn get_bounds(&self) -> Rect {
        return self.widget.bounds;
    }

    fn get_child_bounds(&self) -> Rect {
        return self.widget.get_child_bounds();
    }

    fn add_child(&mut self, mut child: Box<Widget>) {
        // TODO: move this code somewhere else
        let origin: Point = [0.0, 0.0];
        let size: Point = [1000.0, 1000.0];
        let child_bounds = Rect { origin, size };
        child.layout(child_bounds);

        self.widget.children.insert(0, child);
    }

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs, rect: Rect, depth: i32) {
        self.widget.draw(ctx, gl, glyphs, rect, depth);
    }
}