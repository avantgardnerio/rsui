use piston_window::{PistonWindow, WindowSettings, Window};
use sdl2_window::Sdl2Window;

use crate::widget::Widget;

pub struct DrawingWindow<'a> {
    pub window: PistonWindow<Sdl2Window>,
    pub root: &'a Widget,
}

impl<'a> DrawingWindow<'a> {
    pub fn new(root: &'a Widget) -> Self {

        let window: PistonWindow<Sdl2Window> =
             WindowSettings::new("title", [640, 480])
                 .build().unwrap();

//        let window: PistonWindow = WindowSettings::new("Title", [640, 480])
//            .build()
//            .unwrap();
        DrawingWindow {
            window,
            root,
        }
    }

    pub fn run(&mut self) {
        let root = self.root;
        while let Some(event) = self.window.next() {
            let width = self.window.size().width;
            let height = self.window.size().height;
            self.window.draw_2d(&event, |context, gl| {
                root.draw(context, gl, width, height);
            });
//            if let Some(Button::Keyboard(key)) = event.press_args() {
//                self.handle_key_input(key);
//                self.render(&event, window);
//            };
        }
    }

}
