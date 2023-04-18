use fltk::{prelude::*, *};
use fltk_grid::Grid;

struct Panel {
    grid: Grid,
    label: frame::Frame,
    cb1: button::CheckButton,
    cb2: button::CheckButton,
    btn: button::Button,
}
impl Panel {
    pub fn default() -> Self {
        let mut grid = Grid::default();
        grid.debug(false);
        grid.set_layout(6, 1);
        let mut label = frame::Frame::default().with_label("ARTERY:");

        let mut cb1 = button::CheckButton::default().with_label("Normal");
        let mut cb2 = button::CheckButton::default().with_label("Normal");

        let cbvec = vec![cb1.clone(), cb2.clone()];

        let mut btn = button::Button::default().with_label("Submit");
        grid.insert(&mut label, 0, 0);
        grid.insert(&mut cb1, 1, 0);
        grid.insert(&mut cb2, 2, 0);
        grid.insert(&mut btn, 5, 0);
        btn.set_callback(move |btn| {
            for cb in cbvec.clone() {
                println!(
                    "CB status: {}",
                    if cb.is_checked() {
                        "Checked"
                    } else {
                        "Not_checked"
                    }
                );
            }
            println!("--------------");
        });
        Panel {
            grid,
            label,
            cb1,
            cb2,
            btn,
        }
    } // end default fn;
} // end impl Panel;

fltk::widget_extends!(Panel, Grid, grid);

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(800, 600);
    let mut grid = Grid::default_fill();
    grid.debug(false);
    grid.set_layout(1, 2);
    let mut panel1 = Panel::default();
    let mut panel2 = Panel::default();
    grid.insert_grid(&mut *panel1, 0, 0);
    grid.insert_grid(&mut *panel2, 0, 1);
    win.end();
    win.make_resizable(true);
    win.show();

    a.run().unwrap();
}