use graphics::Context;
use opengl_graphics::GlGraphics;
use crate::widget::Widget;

pub struct GridPanel {

}

impl GridPanel {

}

impl Widget for GridPanel {
    fn draw(&self, context: Context, gl: GlGraphics) {
        println!("Widget.draw()");
    }
}