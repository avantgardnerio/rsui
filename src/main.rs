mod drawing_window;
mod grid_panel;
mod widget;

use drawing_window::DrawingWindow;
use grid_panel::GridPanel;
use widget::Widget;

fn main() {
    let mut red_panel: Box<Widget> = Box::new(GridPanel::new([1.0, 0.0, 0.0, 1.0]));

    let mut green_panel: Box<Widget> = Box::new(GridPanel::new([0.0, 1.0, 0.0, 1.0]));

    let mut blue_panel: Box<Widget> = Box::new(GridPanel::new([0.0, 0.0, 1.0, 1.0]));

    green_panel.add_child(blue_panel);

    red_panel.add_child(green_panel);

    let mut app = DrawingWindow::new(red_panel);
    app.run();
}
