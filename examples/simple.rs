use fltk::{prelude::*, *};
use fltk_grid::Grid;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(500, 300);
    let mut grid = Grid::default_fill();
    grid.debug(false); // set to true to show cell outlines and numbers
    grid.set_layout(5, 5); // 5 rows, 5 columns
    grid.insert(&mut button::Button::default(), 0, 1); // widget, row, col
    grid.insert(&mut button::Button::default(), 2..3, 1..4); // widget, row range, col range
                                                             // or
                                                             // grid.insert_ext(&mut button::Button::default(), 2, 1, 3, 1); // widget, row, col, row_span, col_span
    win.end();
    win.show();
    a.run().unwrap();
}
