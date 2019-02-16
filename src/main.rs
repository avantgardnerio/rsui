mod drawing_window;
mod grid_panel;
mod widget;

use drawing_window::DrawingWindow;
use grid_panel::GridPanel;
use widget::Widget;

fn main() {
    let mut red_panel = GridPanel::new([1.0, 0.0, 0.0, 1.0]);

    let green_panel: Box<Widget> = Box::new(GridPanel::new([0.0, 1.0, 0.0, 1.0]));

    red_panel.add_child(green_panel);

    let mut app = DrawingWindow::new(&red_panel);
    app.run();
}
