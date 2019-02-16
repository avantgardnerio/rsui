use piston_window::{G2d, Context, Glyphs};

pub trait Widget {
    fn add_child(&mut self, child: Box<Widget>);

    fn draw(&self, context: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs);
}

pub struct WidgetImpl {
    children: Vec<Box<Widget>>
}

impl WidgetImpl {
    pub fn new() -> Self {
        let children: Vec<Box<Widget>> = Vec::new();
        WidgetImpl {
            children
        }
    }
}

impl Widget for WidgetImpl {
    fn add_child(&mut self, child: Box<Widget>) {
        self.children.push(child)
    }

    fn draw(&self, c: Context, gl: &mut G2d, width: f64, height: f64, glyphs: &mut Glyphs) {
        self.children.iter().for_each(|it| it.draw(c, gl, width, height, glyphs))
    }
}