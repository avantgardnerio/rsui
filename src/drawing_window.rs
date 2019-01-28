use piston_window::{PistonWindow, WindowSettings, Window, Glyphs, TextureSettings};
use glfw_window::GlfwWindow;

use crate::widget::Widget;

pub struct DrawingWindow<'a> {
    pub window: PistonWindow<GlfwWindow>,
    pub root: &'a Widget,
    pub glyphs: Glyphs
}

impl<'a> DrawingWindow<'a> {
    pub fn new(root: &'a Widget) -> Self {

        let window: PistonWindow<GlfwWindow> =
             WindowSettings::new("title", [640, 480])
                 .build().unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        println!("{:?}", assets);
        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        DrawingWindow {
            window,
            root,
            glyphs
        }
    }

    pub fn run(&mut self) {
        let root = self.root;
        let glyphs = &mut self.glyphs;
        while let Some(event) = self.window.next() {
            let width = self.window.size().width;
            let height = self.window.size().height;
            self.window.draw_2d(&event, |context, gl| {
                root.draw(context, gl, width, height, glyphs);
            });
//            if let Some(Button::Keyboard(key)) = event.press_args() {
//                self.handle_key_input(key);
//                self.render(&event, window);
//            };
        }
    }

}
