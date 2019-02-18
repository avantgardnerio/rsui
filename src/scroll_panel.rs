use piston_window::*;

use crate::widget::Widget;
use crate::widget::WidgetImpl;
use crate::widget::Rect;
use crate::h_scroll::HScroll;
use crate::widget::Point;

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
        let my_width: f64 = self.get_bounds().size[0];
        let child_width: f64 = self.get_child_bounds().size[0];
        let diff: f64 = f64::min(0.0, child_width - my_width);
        let max: f64 = diff / my_width;
        println!("ScrollPanel mine={:?} child={:?} max={:?}", my_width, child_width, max);
        self.widget.children.iter_mut().for_each(|child| {
            child.downcast_mut::<HScroll>().map(|h_scroll| h_scroll.set_max(max));
        });
        self.widget.bounds = bounds;
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

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        self.widget.draw(ctx, gl, glyphs)
    }
}