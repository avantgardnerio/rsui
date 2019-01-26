extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use crate::widget::Widget;

pub struct DrawingWindow<'a> {
    pub gl: GlGraphics,
    pub window: Window,
    pub root: &'a Widget
}

impl<'a> DrawingWindow<'a> {
    pub fn new(root: &'a Widget) -> Self {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("Title", [200, 200])
            .opengl(opengl)
            .build()
            .unwrap();
        DrawingWindow {
            gl: GlGraphics::new(opengl),
            window,
            root
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let root = self.root;
        self.gl.draw(args.viewport(), |context, gl: &mut GlGraphics| {
            root.draw(context, gl, args.width, args.height);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
    }
}
