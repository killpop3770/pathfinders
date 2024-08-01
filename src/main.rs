use piston_window::{
    Button, MouseCursorEvent, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

use crate::app::App;
use crate::pathfinder::PathfinderAStar;
use crate::settings::{Settings, Vec2f};

mod app;
mod cell;
mod field;
mod pathfinder;
mod settings;
mod state;

fn main() {
    let settings = Settings::new(
        Vec2f {
            raw_x: 600.0,
            raw_y: 600.0,
        },
        Vec2f {
            raw_x: 20.0,
            raw_y: 20.0,
        },
        30,
    );

    let mut window: PistonWindow = WindowSettings::new(
        "Pathfinder A* test",
        [settings.window_size.raw_x, settings.window_size.raw_y],
    )
    .resizable(false)
    .build()
    .unwrap();

    let algorithm = PathfinderAStar;
    let mut app = App::new(settings, algorithm);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphical_buffer, _| {
            app.render_field(context, graphical_buffer);
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
