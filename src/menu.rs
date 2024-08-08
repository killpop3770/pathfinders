use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;

use piston_window::{CharacterCache, clear, Context, DrawState, G2d, Glyphs, MouseButton, rectangle, text, Transformed};
use piston_window::types::FontSize;

use crate::algorithms::AlgorithmType;
use crate::app::App;
use crate::settings::{Settings, Vec2f};

enum AppState {
    Menu,
    Algorithm,
}

pub struct AppMenu {
    app_state: AppState,
    app: Option<Arc<Mutex<App>>>,
    font_size: FontSize,
    settings: Settings,
    pub items: Vec<UnitAppMenu>,
    mouse_coordinates: Vec2f,
}

impl AppMenu {
    pub fn new(font_size: FontSize, settings: Settings, items: Vec<UnitAppMenu>) -> Self {
        AppMenu {
            app_state: AppState::Menu,
            app: None,
            font_size,
            settings,
            items,
            mouse_coordinates: Vec2f { raw_x: 0.0, raw_y: 0.0 },
        }
    }

    pub fn render(
        &mut self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
    ) {
        match &self.app_state {
            AppState::Menu => {
                clear(piston_window::color::GRAY, g2d);

                let preview_text = "Select algorithm: ";
                let preview_text_width = glyphs.width(self.font_size, preview_text).unwrap();
                text::Text::new_color(piston_window::color::BLACK, self.font_size)
                    .draw(
                        preview_text,
                        glyphs,
                        &DrawState::default(),
                        context.transform.trans((self.settings.window_size.raw_x - preview_text_width) / 2.0, 100.0),
                        g2d,
                    )
                    .unwrap();

                let padding = 20.0;// between units
                let mut temp = 70.0;// upper padding
                for item in self.items.iter_mut() {
                    item.width = self.settings.window_size.raw_x / 2.0;
                    item.height = self.settings.window_size.raw_y / 10.0;
                    temp += item.height + padding;
                    item.x = self.settings.window_size.raw_x / 2.0;
                    item.y = temp;
                    item.render(context, g2d, glyphs, self.font_size);
                }
            }
            AppState::Algorithm => {
                if let Some(app_ref) = &self.app {
                    let mut app = app_ref.lock().unwrap();
                    app.render(context, g2d, glyphs);
                };
            }
        }
    }

    pub fn back_to_menu(&mut self) {
        if let Some(app_ref) = &self.app {
            let app_guard = app_ref.lock().unwrap();
            app_guard.should_stop.store(true, Ordering::Relaxed);
            self.app_state = AppState::Menu;
        };
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            let x = self.mouse_coordinates.raw_x;
            let y = self.mouse_coordinates.raw_y;
            self.items.iter()
                .filter(|item| item.is_hovered(x, y))
                .map(|item| {
                    self.app_state = AppState::Algorithm;
                    self.app = Some(Arc::new(Mutex::new(item.run(self.settings))))
                })
                .count();
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coordinates.raw_x = args[0];
        self.mouse_coordinates.raw_y = args[1];
    }
}

pub struct UnitAppMenu {
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    algorithm_type: AlgorithmType,
}

impl UnitAppMenu {
    pub fn new(label: &str, algorithm_type: AlgorithmType) -> Self {
        UnitAppMenu {
            label: label.to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            algorithm_type,
        }
    }

    pub fn run(&self, settings: Settings) -> App {
        let mut app = App::new(settings.clone(), &self.algorithm_type);
        app.start();
        return app;
    }

    pub fn render(
        &mut self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
        font_size: FontSize,
    ) {
        let unit_x = self.x - (self.width / 2.0);
        let unit_y = self.y;
        rectangle(
            piston_window::color::CYAN,
            [
                unit_x,
                unit_y,
                self.width,
                self.height,
            ],
            context.transform,
            g2d,
        );
        let transform = context.transform.trans(unit_x, unit_y);
        let unit_text = &*self.label;
        let text_width = glyphs.width(font_size, unit_text).unwrap();
        text::Text::new_color(piston_window::color::BLACK, font_size)
            .draw(
                unit_text,
                glyphs,
                &DrawState::default(),
                transform.trans(
                    (self.width - text_width) / 2.0,
                    (self.height + font_size as f64 * 0.75) / 2.0,
                ),
                g2d,
            )
            .unwrap();
    }

    fn is_hovered(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}