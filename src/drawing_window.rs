use piston_window::{PistonWindow, WindowSettings, Window, Glyphs, TextureSettings};
use glfw_window::GlfwWindow;
use piston_window::{Event, Input};

use crate::widget::Widget;
use crate::widget::Rect;

pub struct DrawingWindow {
    pub window: PistonWindow<GlfwWindow>,
    pub root: Box<Widget>,
    pub glyphs: Glyphs
}

impl DrawingWindow {
    pub fn new(root: Box<Widget>) -> Self {

        let window: PistonWindow<GlfwWindow> =
             WindowSettings::new("title", [640, 480])
                 .build().unwrap();

        let font_data: &[u8] = include_bytes!("../assets/FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let glyphs = Glyphs::from_bytes(font_data, factory, TextureSettings::new()).unwrap();

        DrawingWindow {
            window,
            root,
            glyphs
        }
    }

    pub fn run(&mut self) {
        let mut first = true;
        let root = &mut self.root;
        let glyphs = &mut self.glyphs;
        while let Some(event) = self.window.next() {
            let width = self.window.size().width;
            let height = self.window.size().height;
            if(first) {
                let rect = Rect {
                    origin: [0.0, 0.0],
                    size: [width as f64, height as f64]
                };
                root.layout(rect);
                first = false;
            }
            self.window.draw_2d(&event, |context, gl| {
                root.draw(context, gl, width, height, glyphs);
            });
            match event {
                Event::Input(ref input) => {
                    match input {
                        Input::Resize(ref x, ref y) => {
                            let rect = Rect {
                                origin: [0.0, 0.0],
                                size: [x.clone(), y.clone()]
                            };
                            root.layout(rect);
                            println!("resize {} {}", x, y);
                        },
                        _ => {}
                    }
                },
                Event::Loop(ref l) => {
                },
                Event::Custom(ref id, _) => {
                }
            }
//            if let Some(Button::Keyboard(key)) = event.press_args() {
//                self.handle_key_input(key);
//                self.render(&event, window);
//            };
        }
    }

}
