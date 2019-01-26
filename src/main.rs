mod drawing_window;
mod grid_panel;
mod widget;

use drawing_window::DrawingWindow;
use grid_panel::GridPanel;

fn main() {
    let red_panel = GridPanel::new([1.0, 0.0, 0.0, 1.0]);
    let mut app = DrawingWindow::new(&red_panel);
    app.run();
}
