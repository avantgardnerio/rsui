use piston_window::*;

pub type Point = [f64; 2];

pub type Color = [f32; 4];

#[derive(Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Point
}

pub trait Widget {
    fn layout(&mut self, bounds: Rect);

    fn get_bounds(&self) -> Rect;
    
    fn add_child(&mut self, child: Box<Widget>);

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs);
}

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
    }

    fn add_child(&mut self, child: Box<Widget>) {
        self.children.push(child)
    }

    fn get_bounds(&self) -> Rect {
        return self.bounds;
    }

    fn draw(&self, ctx: Context, gl: &mut G2d, glyphs: &mut Glyphs) {
        self.children.iter().for_each(|child| {
            let bounds = child.get_bounds();
            let trans = ctx.transform.trans(bounds.origin[0], bounds.origin[1]);
            let viewport = ctx.viewport.unwrap();
            let scaleX = viewport.draw_size[0] as f64 / viewport.window_size[0];
            let scaleY = viewport.draw_size[1] as f64 / viewport.window_size[1];
            let rect = [
                (bounds.origin[0] * scaleX) as u32,
                (bounds.origin[1] * scaleY) as u32,
                (bounds.size[0] * scaleX) as u32,
                (bounds.size[1] * scaleY) as u32
            ];
            let clipped = Context {
                viewport: ctx.viewport,
                view: ctx.view,
                transform: trans,
                draw_state: ctx.draw_state.scissor(rect)
            };
            child.draw(clipped, gl, glyphs);
        })
    }
}