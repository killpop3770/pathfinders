use std::path::Path;
use std::sync::{Arc, Mutex};

use piston_window::{Button, Key, MouseCursorEvent, PistonWindow, PressEvent, WindowSettings};

use crate::algorithms::AlgorithmType;
use crate::menu::{AppMenu, AppState, UnitAppMenu};
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
                Button::Keyboard(key) => {
                    match key {
                        Key::Escape => app_menu.back_to_menu(),
                        Key::D1 | Key::NumPad1 => {
                            app_menu.app_state = AppState::Algorithm;
                            app_menu.app = Some(Arc::new(Mutex::new(app_menu.items.get(0).unwrap().run(app_menu.settings))));
                        }
                        Key::D2 | Key::NumPad2 => {
                            app_menu.app_state = AppState::Algorithm;
                            app_menu.app = Some(Arc::new(Mutex::new(app_menu.items.get(1).unwrap().run(app_menu.settings))));
                        }
                        Key::D3 | Key::NumPad3 => {
                            app_menu.app_state = AppState::Algorithm;
                            app_menu.app = Some(Arc::new(Mutex::new(app_menu.items.get(2).unwrap().run(app_menu.settings))));
                        }
                        Key::D4 | Key::NumPad4 => {
                            app_menu.app_state = AppState::Algorithm;
                            app_menu.app = Some(Arc::new(Mutex::new(app_menu.items.get(3).unwrap().run(app_menu.settings))));
                        }
                        Key::D5 | Key::NumPad5 => {
                            app_menu.app_state = AppState::Algorithm;
                            app_menu.app = Some(Arc::new(Mutex::new(app_menu.items.get(4).unwrap().run(app_menu.settings))));
                        }
                        _ => {}
                    }
                }
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
