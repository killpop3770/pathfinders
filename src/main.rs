use std::path::Path;

use piston_window::{Button, MouseCursorEvent, PistonWindow, PressEvent, WindowSettings};

use crate::algorithms::AlgorithmType;
use crate::menu::{AppMenu, UnitAppMenu};
use crate::settings::Settings;

mod algorithms;
mod app;
mod cell;
mod field;
mod settings;
mod state;
mod menu;
mod colors;

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

    let mut app_menu = AppMenu::new(
        35,
        settings,
        vec![
            UnitAppMenu::new("BFS", AlgorithmType::BFS),
            UnitAppMenu::new("DFS", AlgorithmType::DFS),
            UnitAppMenu::new("GBFS", AlgorithmType::GBFS),
            UnitAppMenu::new("DIJKSTRA", AlgorithmType::Dijkstra),
            UnitAppMenu::new("A_STAR", AlgorithmType::AStar),
        ],
    );


    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphical_buffer, device| {
            app_menu.render(context, graphical_buffer, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        // if let Some(ref args) = event.update_args() {
        //     app.update(args);
        // }

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(_) => {}
                Button::Mouse(mouse_button) => {
                    app_menu.on_mouse_click(&mouse_button);
                }
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        }

        if let Some(move_args) = event.mouse_cursor_args() {
            app_menu.on_mouse_move(&move_args);
        }
    }
}
