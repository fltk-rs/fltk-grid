/*!
# fltk-grid

A grid widget for fltk-rs.

## Usage
```toml,no_run
[dependencies]
fltk = "1.3"
fltk-grid = "0.1"
```

```rust,no_run
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
- debug(): shows the cell outline and their numbering, useful for prototyping.
- resize(): determines how the grid is resized.
*/

#![allow(clippy::needless_doctest_main)]

use fltk::{prelude::*, *};
use std::collections::HashMap;

/// A grid widget
#[derive(Debug, Clone)]
pub struct Grid {
    table: table::Table,
    rows: i32,
    cols: i32,
    widgets: HashMap<(i32, i32, i32, i32), widget::Widget>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(0, 0, 0, 0, None)
    }
}

impl Grid {
    /// Creates a new grid widget
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let mut table = table::Table::new(x, y, w, h, label);
        table.set_frame(enums::FrameType::NoBox);
        table.set_scrollbar_size(-1);
        table.end();
        Self {
            table,
            rows: 0,
            cols: 0,
            widgets: HashMap::default(),
        }
    }

    /// Creates a default value grid widget filling the parent
    pub fn default_fill() -> Self {
        let g = Grid::default();
        Self {
            table: g.table.size_of_parent().center_of_parent(),
            rows: 0,
            cols: 0,
            widgets: HashMap::default(),
        }
    }

    /// Sets the rows and columns of the widget
    pub fn set_layout(&mut self, rows: i32, cols: i32) {
        self.rows = rows;
        self.cols = cols;
        self.table.set_rows(rows);
        self.table.set_cols(cols);
        let parent = self.table.parent().unwrap();
        self.table.set_row_height_all(parent.h() / rows);
        self.table.set_col_width_all(parent.w() / cols);
    }

    /// Adds a widget to the grid
    pub fn insert_ext<W: 'static + Clone + WidgetExt>(
        &mut self,
        widget: &mut W,
        row: i32,
        col: i32,
        row_span: i32,
        col_span: i32,
    ) {
        if let Some((x, y, w, h)) = self.table.find_cell(table::TableContext::Cell, row, col) {
            widget.resize(x, y, w * row_span, h * col_span);
            self.table.add(widget);
            self.widgets.insert((row, col, row_span, col_span), unsafe {
                widget.into_widget()
            });
        }
    }

    /// Insert a widget with a single span
    pub fn insert<W: 'static + Clone + WidgetExt>(&mut self, widget: &mut W, row: i32, col: i32) {
        self.insert_ext(widget, row, col, 1, 1);
    }

    /// Removes a widget
    pub fn remove<W: WidgetExt>(&mut self, widget: &mut W) {
        self.table.remove(widget);
    }

    /// Determine how a grid is resized
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.table.resize(x, y, w, h);
        let rows = self.rows;
        let cols = self.cols;
        let parent = self.table.parent().unwrap();
        self.table.set_row_height_all(parent.h() / rows);
        self.table.set_col_width_all(parent.w() / cols);
        for wi in &mut self.widgets {
            if let Some((x, y, w, h)) =
                self.table
                    .find_cell(table::TableContext::Cell, wi.0 .0, wi.0 .1)
            {
                wi.1.resize(x, y, w * wi.0 .2, h * wi.0 .3);
            }
        }
    }

    /// Show cell outlines and numbering
    pub fn debug(&mut self, flag: bool) {
        if flag {
            self.table.draw_cell(move |_, ctx, row, col, x, y, w, h| {
                if ctx == table::TableContext::Cell {
                    draw::set_draw_color(enums::Color::Red);
                    draw::draw_rect(x, y, w, h);
                    draw::draw_text2(
                        &format!("{},{}", row, col),
                        x,
                        y,
                        w,
                        h,
                        enums::Align::Center,
                    );
                }
            });
        }
    }
}
