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
        let mut title = frame::Frame::default().with_label("Employee Form");
        title.set_frame(enums::FrameType::FlatBox);
        title.set_color(enums::Color::Red);
        title.set_label_color(enums::Color::White);
        grid.insert(
            // insert widgets
            &mut title, 0, 1..4
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
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(500, 300);
    let mut form = Form::default();
    form.register_default_callback();
    win.end();
    win.make_resizable(true);
    win.show();

    win.resize_callback(move |_, _, _, w, h| form.resize(0, 0, w, h));

    a.run().unwrap();
}
