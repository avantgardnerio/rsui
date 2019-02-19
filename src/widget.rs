use piston_window::*;

pub use vecmath::vec2_scale as mul;
pub use vecmath::vec2_add as add;

use crate::geom::Rect;

pub trait Widget: mopa::Any {
    fn layout(&mut self, bounds: Rect);

    fn get_bounds(&self) -> Rect;

    fn get_child_bounds(&self) -> Rect;
    
    fn add_child(&mut self, child: Box<Widget>);

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs, rect: Rect, depth: i32);
}

mopafy!(Widget);

pub struct WidgetImpl {
    pub bounds: Rect,
    pub children: Vec<Box<Widget>>
}

impl WidgetImpl {
    pub fn new() -> Self {
        let children: Vec<Box<Widget>> = Vec::new();
        let bounds = Rect {
            origin: [0.0, 0.0],
            size: [0.0, 0.0]
        };
        WidgetImpl {
            bounds,
            children
        }
    }
}

impl Widget for WidgetImpl {
    fn layout(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.children.iter_mut().for_each(|child| {
            let child_bounds = Rect {
                origin: [0.0, 0.0],
                size: bounds.size
            };
            child.layout(child_bounds);
        });
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.children.push(child)
    }

    fn get_bounds(&self) -> Rect {
        return self.bounds;
    }

    fn get_child_bounds(&self) -> Rect {
        let result = self.children.iter().fold(Rect::min(), |acc, cur| {
            return Rect::union(acc, cur.get_bounds());
        });
        return result;
    }

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs, rect: Rect, depth: i32) {
        let self_bounds = self.get_bounds()
            .translate(mul(self.get_bounds().origin, -1.0))
            .translate(rect.origin);
        self.children.iter().for_each(|child| {
            let child_bounds = child.get_bounds().translate(rect.origin);
            let trans = ctx.transform.trans(
                child.get_bounds().origin[0],
                child.get_bounds().origin[1]
            );
            let clip_bounds = Rect::isec(rect, Rect::isec(self_bounds, child_bounds));
            let clip_rect = clip_bounds.to_u32();
            let clipped = Context {
                viewport: ctx.viewport,
                view: ctx.view,
                transform: trans,
                draw_state: ctx.draw_state.scissor(clip_rect)
            };
            child.draw(clipped, gl, glyphs, clip_bounds, depth + 1);
        })
    }
}