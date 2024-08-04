use crate::algorithms::greedy_best_first_search::GBFS;
use piston_window::{
    Button, MouseCursorEvent, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

use crate::algorithms::breadth_first_search::BFS;
use crate::algorithms::depth_first_search::DFS;
use crate::app::App;
use crate::settings::Settings;

mod algorithms;
mod app;
mod cell;
mod field;
mod settings;
mod state;

fn main() {
    let settings = Settings::new(20, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Pathfinder A* test",
        [settings.window_size.raw_x, settings.window_size.raw_y],
    )
    .resizable(false)
    .vsync(true)
    .build()
    .unwrap();

    // let algorithm = Box::new(BFS);
    // let algorithm = Box::new(DFS);
    let algorithm = Box::new(GBFS);
    let mut app = App::new(settings, algorithm);

    //TODO: create menu
    //TODO: bind keys(space, etc..)
    //TODO: complete speed factor
    //TODO: handle error from threads
    //TODO: complete bfs, dfs, dijkstra algorithms
    //TODO: check coords type bound while casting

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphical_buffer, _| {
            app.render(context, graphical_buffer);
            // println!("render");
        });

        if let Some(ref args) = event.update_args() {
            // println!("update");
            // app.update(args);
        }

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
