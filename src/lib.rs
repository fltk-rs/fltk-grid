/*!
# fltk-grid

A grid widget for fltk-rs.

## Usage
```toml,no_run
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


```rust,no_run
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
    pub fn remove<W: WidgetExt>(&mut self, widget: &W) {
        self.widgets.retain(|_, v| unsafe { v.as_widget_ptr() != widget.as_widget_ptr() });
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
                    draw::set_font(enums::Font::Helvetica, 14);
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
