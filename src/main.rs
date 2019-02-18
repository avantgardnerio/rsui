#[macro_use]
extern crate mopa;

mod drawing_window;
mod grid_panel;
mod widget;
mod scroll_panel;
mod h_scroll;

use drawing_window::DrawingWindow;
use grid_panel::GridPanel;
use widget::Widget;
use scroll_panel::ScrollPanel;

fn main() {
    let mut red_scroll = Box::new(ScrollPanel::new());
    let mut green_scroll = Box::new(ScrollPanel::new());

    let mut red_panel = Box::new(GridPanel::new([1.0, 0.0, 0.0, 1.0]));
    let mut green_panel = Box::new(GridPanel::new([0.0, 1.0, 0.0, 1.0]));
    let blue_panel = Box::new(GridPanel::new([0.0, 0.0, 1.0, 1.0]));

    green_scroll.add_child(blue_panel);
    green_panel.add_child(green_scroll);
    red_scroll.add_child(green_panel);
    red_panel.add_child(red_scroll);

    let mut app = DrawingWindow::new(red_panel);
    app.run();
}
