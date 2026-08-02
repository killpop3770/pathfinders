use piston_window::types::FontSize;
use piston_window::{
    clear, rectangle, text, CharacterCache, Context, DrawState, G2d, Glyphs, MouseButton,
    Transformed,
};

use crate::algorithms::AlgorithmType;
use crate::config::settings::AppConfig;
use crate::presentation::app::App;

pub enum AppState {
    Menu,
    Algorithm,
}

pub struct AppMenu {
    pub app_state: AppState,
    pub app: Option<App>,
    font_size: FontSize,
    config: AppConfig,
    pub items: Vec<UnitAppMenu>,
    mouse_x: f64,
    mouse_y: f64,
}

impl AppMenu {
    pub fn new(font_size: FontSize, config: AppConfig, items: Vec<UnitAppMenu>) -> Self {
        AppMenu {
            app_state: AppState::Menu,
            app: None,
            font_size,
            config,
            items,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn render(&mut self, context: Context, g2d: &mut G2d, glyphs: &mut Glyphs) {
        match &self.app_state {
            AppState::Menu => {
                clear(piston_window::color::GRAY, g2d);

                let preview_text = "Select algorithm: ";
                let preview_text_width = glyphs.width(self.font_size, preview_text).unwrap_or(0.0);
                let (win_w, _) = self.config.window_size();

                text::Text::new_color(piston_window::color::BLACK, self.font_size)
                    .draw(
                        preview_text,
                        glyphs,
                        &DrawState::default(),
                        context
                            .transform
                            .trans((win_w - preview_text_width) / 2.0, 100.0),
                        g2d,
                    )
                    .ok();

                let padding = 20.0;
                let mut current_y = 70.0;

                for item in &mut self.items {
                    item.width = win_w / 2.0;
                    item.height = self.config.window_size().1 / 10.0;
                    current_y += item.height + padding;
                    item.x = win_w / 2.0;
                    item.y = current_y;
                    item.render(context, g2d, glyphs, self.font_size);
                }
            }
            AppState::Algorithm => {
                if let Some(app) = &mut self.app {
                    app.render(context, g2d, glyphs);
                }
            }
        }
    }

    pub fn start_algorithm_by_index(&mut self, index: usize) {
        if let Some(item) = self.items.get(index) {
            self.app_state = AppState::Algorithm;
            let mut app = item.run(self.config.clone());
            app.start();
            self.app = Some(app);
        }
    }

    pub fn back_to_menu(&mut self) {
        if let Some(mut app) = self.app.take() {
            app.stop();
        }
        self.app_state = AppState::Menu;
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        if let MouseButton::Left = button {
            if let Some(item) = self
                .items
                .iter()
                .find(|item| item.is_hovered(self.mouse_x, self.mouse_y))
            {
                self.app_state = AppState::Algorithm;
                let mut app = item.run(self.config.clone());
                app.start();
                self.app = Some(app);
            }
        }
    }

    pub fn on_mouse_move(&mut self, pos: [f64; 2]) {
        self.mouse_x = pos[0];
        self.mouse_y = pos[1];
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

    pub fn run(&self, config: AppConfig) -> App {
        let mut app = App::new(config, &self.algorithm_type);
        app.start();
        app
    }

    pub fn render(
        &self,
        context: Context,
        g2d: &mut G2d,
        glyphs: &mut Glyphs,
        font_size: FontSize,
    ) {
        let unit_x = self.x - (self.width / 2.0);
        let unit_y = self.y;

        rectangle(
            piston_window::color::CYAN,
            [unit_x, unit_y, self.width, self.height],
            context.transform,
            g2d,
        );

        let transform = context.transform.trans(unit_x, unit_y);
        let text_width = glyphs.width(font_size, &self.label).unwrap_or(0.0);

        text::Text::new_color(piston_window::color::BLACK, font_size)
            .draw(
                &self.label,
                glyphs,
                &DrawState::default(),
                transform.trans(
                    (self.width - text_width) / 2.0,
                    (self.height + font_size as f64 * 0.75) / 2.0,
                ),
                g2d,
            )
            .ok();
    }

    fn is_hovered(&self, x: f64, y: f64) -> bool {
        let unit_x = self.x - (self.width / 2.0);
        x >= unit_x && x <= unit_x + self.width && y >= self.y && y <= self.y + self.height
    }
}
