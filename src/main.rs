use std::path::Path;

use piston_window::{Button, MouseCursorEvent, PistonWindow, PressEvent, WindowSettings};

use crate::algorithms::a_star::AStar;
use crate::app::App;
use crate::settings::Settings;

mod algorithms;
mod app;
mod cell;
mod field;
mod settings;
mod state;

fn main() {
    let settings = Settings::new(30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Pathfinders test",
        [settings.window_size.raw_x, settings.window_size.raw_y],
    )
    .resizable(false)
    .vsync(true)
    .build()
    .expect("Can not create a main window!");

    let mut glyphs = window
        .load_font(Path::new("./assets/Particle-Regular.otf"))
        .expect("Can not find fonts!");

    // let algorithm = Box::new(BFS);
    // let algorithm = Box::new(DFS);
    // let algorithm = Box::new(GBFS);
    // let algorithm = Box::new(Dijkstra);
    let algorithm = Box::new(AStar);
    let mut app = App::new(settings, algorithm);

    //TODO: create menu
    //TODO: bind keys(space, etc..) and mouse
    //TODO: create README.md

    //TODO: handle error from threads
    //TODO: is i16/u16 enough?
    //TODO: make gradient for cell cost?
    //TODO: make one/two default map/maze for all algorithms?

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphical_buffer, device| {
            app.render(context, graphical_buffer, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        // if let Some(ref args) = event.update_args() {
        //     app.update(args);
        // }

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(_) => {}
                Button::Mouse(mouse_button) => {
                    app.on_mouse_click(&mouse_button);
                }
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        }

        if let Some(move_args) = event.mouse_cursor_args() {
            app.on_mouse_move(&move_args);
        }
    }
}
