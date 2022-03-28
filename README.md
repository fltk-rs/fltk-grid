# fltk-grid

A grid widget for fltk-rs.

## Usage
```toml
[dependencies]
fltk = "1.3"
fltk-grid = "0.1"
```

Basically, the crate contains a single type Grid which has 4 main non-constructor methods:
- set_layout(): specifies the number of rows and columns of the grid.
- insert(): specifies the widget to be inserted, along with in which cell (row, column).
- insert_ext(): adds to insert the row span and column span.
- resize(): determines how the grid is resized.
- debug(): shows the cell outline and their numbering, useful for prototyping. 


```rust
use fltk::{prelude::*, *};
use fltk_grid::Grid;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(500, 300);
    let mut grid = Grid::default_fill();
    grid.debug(false); // set to true to show cell outlines and numbers
    grid.set_layout(5, 5); // 5 rows, 5 columns
    grid.insert(&mut button::Button::default(), 0, 1); // widget, row, col
    grid.insert_ext(&mut button::Button::default(), 2, 1, 3, 1); // widget, row, col, row_span, col_span
    win.end();
    win.show();
    a.run().unwrap();
}
```

## Example
Run `cargo run --example form`

- [Form example](https://github.com/fltk-rs/fltk-grid/blob/main/examples/form.rs)

![image](https://user-images.githubusercontent.com/37966791/160343002-06763412-4c94-4955-9444-4d2ba533e3ac.png)

Setting Grid::debug(true):

![image](https://user-images.githubusercontent.com/37966791/160343165-a96161a1-4f86-4995-a990-e78b72f7d109.png)