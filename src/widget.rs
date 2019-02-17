use piston_window::{G2d, Context, Glyphs};

pub type Point = [f64; 2];

pub type Color = [f32; 4];

#[derive(Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub size: Point
}

pub trait Widget {
    fn layout(&mut self, bounds: Rect);
    
    fn add_child(&mut self, child: Box<Widget>);

    fn draw(&self, context: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs);
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

    fn draw(&self, c: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs) {
        self.children.iter().for_each(|it| it.draw(c, gl, width, height, glyphs))
    }
}