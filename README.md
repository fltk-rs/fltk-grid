# fltk-grid

A grid widget for fltk-rs.

![image](https://user-images.githubusercontent.com/37966791/160294170-d3361c11-76bd-447e-a1eb-17474e222dc3.png)

## Usage
```toml
[dependencies]
fltk = "1.3"
fltk-grid = "0.1"
```

```rust
use fltk::{prelude::*, *};
use fltk_grid::Grid;

struct Form {
    grid: Grid,
    name: input::Input,
    age: input::IntInput,
    occupation: input::Input,
    btn: button::Button,
}

impl Form {
    pub fn default() -> Self {
        let mut grid = Grid::default_fill();
        grid.set_layout(10, 5); // construct a new grid
        let name = input::Input::default();
        let age = input::IntInput::default();
        let occupation = input::Input::default();
        let btn = button::Button::default().with_label("Submit");
        let mut g = Self {
            grid,
            name,
            age,
            occupation,
            btn,
        };
        g.fill();
        g
    }

    fn fill(&mut self) {
        let grid = &mut self.grid;
        grid.debug(false); // set to true to see cell outlines
        grid.insert_ext(
            // insert widgets
            &mut frame::Frame::default().with_label("Employee Form"),
            0,
            1,
            3,
            1,
        );
        grid.insert(&mut frame::Frame::default().with_label("Name"), 2, 1);
        grid.insert(&mut self.name, 2, 3);
        grid.insert(&mut frame::Frame::default().with_label("Age"), 4, 1);
        grid.insert(&mut self.age, 4, 3);
        grid.insert(&mut frame::Frame::default().with_label("Occupation"), 6, 1);
        grid.insert(&mut self.occupation, 6, 3);
        grid.insert(&mut self.btn, 8, 2);
    }

    fn register_default_callback(&mut self) {
        self.btn.set_callback({
            let name = self.name.clone();
            let age = self.age.clone();
            let occupation = self.occupation.clone();
            move |_| {
                println!("Name: {}", name.value());
                println!("Age: {}", age.value());
                println!("Occupation: {}", occupation.value());
            }
        });
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.grid.resize(x, y, w, h); // determine how it's resized
    }
}

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(500, 400);
    let mut form = Form::default();
    form.register_default_callback();
    win.end();
    win.make_resizable(true);
    win.show();

    win.resize_callback(move |_, _, _, w, h| form.resize(0, 0, w, h));

    a.run().unwrap();
}
```

Basically, the crate contains a single type Grid which has 4 main non-constructor methods:
- set_layout(): specifies the number of rows and columns of the grid.
- insert(): specifies the widget to be inserted, along with in which cell (row, column).
- insert_ext(): adds to insert the row span and column span.
- resize(): determines how the grid is resized.
- debug(): shows the cell outline and their numbering, useful for prototyping. Setting debug to true shows:

![image](https://user-images.githubusercontent.com/37966791/160294219-c9d0fd32-6bea-4867-b1f6-d44aa49fe077.png)

