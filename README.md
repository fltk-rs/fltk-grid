# fltk-grid

A grid widget for fltk-rs. This crate exists for backwards compatibility, since fltk 1.5 provides a Grid widget (under `fltk::group::Grid`).

## Usage
```toml
[dependencies]
fltk = "1.5.0"
fltk-grid = "0.4"
```

Basically, the crate contains a single type Grid which has 4 main non-constructor methods:
- set_layout(): specifies the number of rows and columns of the grid.
- set_widget(): specifies the widget to be placed, along with in which cell (row, column). The values can be a range (0..1).
- set_widget_ext(): adds to set_widget the row span and column span, in addition to the Grid Alignment.
- resize(): determines how the grid is resized.
- debug(): shows the cell outline and their numbering, useful for prototyping. 


```rust
use fltk::{prelude::*, *};
use fltk_grid::Grid;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(500, 300);
    let mut grid = Grid::default_fill();
    grid.show_grid(false); // set to true to show cell outlines and numbers
    grid.set_layout(5, 5); // 5 rows, 5 columns
    grid.set_widget(&mut button::Button::default(), 0, 1); // widget, row, col
    grid.set_widget(&mut button::Button::default(), 2..3, 1..4); // widget, row range, col range
    // or
    // grid.set_widget_ext(&mut button::Button::default(), 2, 1, 1, 3, GridAlign::FILL); // widget, row, col, row_span, col_span
    grid.end();
    win.end();
    win.show();
    a.run().unwrap();
}
```

## Example
Run `cargo run --example form`

- [Form example](https://github.com/fltk-rs/fltk-grid/blob/main/examples/form.rs)

![image](https://user-images.githubusercontent.com/37966791/160347418-b8b54408-3dc9-4fc4-93e8-fb6c1c0282e9.png)

Setting Grid::debug(true):

![image](https://user-images.githubusercontent.com/37966791/160346084-f3b0dad7-bd14-41da-99a0-768f7327ab2c.png)
