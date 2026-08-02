use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{
    Button, Key, MouseCursorEvent, PistonWindow, PressEvent, TextureSettings, WindowSettings,
};

use crate::algorithms::AlgorithmType;
use crate::config::constants::APP_TITLE;
use crate::config::settings::AppConfig;
use crate::presentation::menu::{AppMenu, AppState, UnitAppMenu};

mod algorithms;
mod config;
mod domain;
mod presentation;
mod state;

fn main() {
    env_logger::builder()
        .filter_module("gfx_device_gl", log::LevelFilter::Off)
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("App started");

    let config = AppConfig::load();
    let (win_w, win_h) = config.window_size();
    log::info!("Config initialized");

    let mut window: PistonWindow = WindowSettings::new(APP_TITLE, [win_w, win_h])
        .resizable(false)
        .vsync(true)
        .build()
        .expect("Failed to create main window");

    let mut glyphs = GlyphCache::from_bytes(
        include_bytes!("../assets/Particle-Regular.otf"),
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Failed to load font");

    let mut app_menu = AppMenu::new(
        35,
        config.clone(),
        vec![
            UnitAppMenu::new("BFS", AlgorithmType::BFS),
            UnitAppMenu::new("DFS", AlgorithmType::DFS),
            UnitAppMenu::new("GBFS", AlgorithmType::GBFS),
            UnitAppMenu::new("DIJKSTRA", AlgorithmType::Dijkstra),
            UnitAppMenu::new("A_STAR", AlgorithmType::AStar),
        ],
    );
    log::info!("Menu initialized");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, g2d, device| {
            match app_menu.app_state {
                AppState::Menu => {
                    app_menu.render(context, g2d, &mut glyphs);
                }
                AppState::Algorithm => {
                    if let Some(app) = &mut app_menu.app {
                        app.render(context, g2d, &mut glyphs);
                    }
                }
            }
            glyphs.factory.encoder.flush(device);
        });

        if let Some(button) = event.press_args() {
            match app_menu.app_state {
                AppState::Menu => match button {
                    Button::Keyboard(Key::D1 | Key::NumPad1) => {
                        app_menu.start_algorithm_by_index(0);
                    }
                    Button::Keyboard(Key::D2 | Key::NumPad2) => {
                        app_menu.start_algorithm_by_index(1);
                    }
                    Button::Keyboard(Key::D3 | Key::NumPad3) => {
                        app_menu.start_algorithm_by_index(2);
                    }
                    Button::Keyboard(Key::D4 | Key::NumPad4) => {
                        app_menu.start_algorithm_by_index(3);
                    }
                    Button::Keyboard(Key::D5 | Key::NumPad5) => {
                        app_menu.start_algorithm_by_index(4);
                    }
                    Button::Mouse(mouse_button) => {
                        app_menu.on_mouse_click(&mouse_button);
                    }
                    _ => {}
                },
                AppState::Algorithm => match button {
                    Button::Keyboard(Key::Escape) => {
                        app_menu.back_to_menu();
                    }
                    Button::Mouse(mouse_button) => {
                        if let Some(app) = &mut app_menu.app {
                            app.on_mouse_click(&mouse_button);
                        }
                    }
                    _ => {}
                },
            }
        }

        if let Some(cursor_pos) = event.mouse_cursor_args() {
            match app_menu.app_state {
                AppState::Menu => {
                    app_menu.on_mouse_move(cursor_pos);
                }
                AppState::Algorithm => {
                    if let Some(app) = &mut app_menu.app {
                        app.on_mouse_move(&cursor_pos);
                    }
                }
            }
        }
    }
}
