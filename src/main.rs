use piston_window::{Button, MouseCursorEvent, PistonWindow, PressEvent, WindowSettings};

use crate::app::App;
use crate::settings::{Settings, Vec2f};

mod settings;
mod app;
mod field;

fn main() {
    let settings = Settings::new
        (
            Vec2f { x: 400.0, y: 400.0 },
            Vec2f { x: 20.0, y: 20.0 },
            20,
        );

    let mut window: PistonWindow = WindowSettings::new
        (
            "Pathfinder A* test",
            [settings.window_size.x, settings.window_size.y],
        )
        .resizable(false)
        .build().unwrap();

    let mut app = App::new(settings);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphical_buffer, _| {
            app.render_field(context, graphical_buffer);
        });

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