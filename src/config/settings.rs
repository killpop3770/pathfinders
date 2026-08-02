use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::config::constants::DEFAULT_CONFIG_PATH;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub grid: GridConfig,
    pub animation: AnimationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridConfig {
    pub cells_number: u16,
    pub cell_size: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimationConfig {
    pub default_speed: f64,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = Path::new(DEFAULT_CONFIG_PATH);

        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(contents) => match toml::from_str::<Self>(&contents) {
                    Ok(config) => {
                        log::info!("Successfully load config from {}", DEFAULT_CONFIG_PATH);
                        return config;
                    }
                    Err(e) => {
                        log::warn!(
                            "Parse error {}: {}. Default config is used.",
                            DEFAULT_CONFIG_PATH,
                            e
                        );
                    }
                },
                Err(e) => {
                    log::warn!(
                        "Can not read {}: {}. Default config is used.",
                        DEFAULT_CONFIG_PATH,
                        e
                    );
                }
            }
        } else {
            log::info!(
                "Can not find file with name: {}. Default config is used.",
                DEFAULT_CONFIG_PATH
            );
        }

        Self::default()
    }

    pub fn cells_number(&self) -> u16 {
        self.grid.cells_number
    }

    pub fn cell_size(&self) -> u16 {
        self.grid.cell_size
    }

    pub fn window_size(&self) -> (f64, f64) {
        (self.window.width, self.window.height)
    }

    pub fn default_speed(&self) -> f64 {
        self.animation.default_speed
    }

    pub fn font_size(&self) -> u32 {
        (self.grid.cell_size as f64 * 0.65) as u32
    }

    pub fn cell_offset_x(&self) -> f64 {
        let font_size = self.font_size() as f64;
        (self.grid.cell_size as f64) / 2.0 - (font_size / 2.0)
    }

    pub fn cell_offset_y(&self) -> f64 {
        let font_size = self.font_size() as f64;
        (self.grid.cell_size as f64) / 2.0 + (font_size / 2.5)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 800.0,
                height: 800.0,
            },
            grid: GridConfig {
                cells_number: 25,
                cell_size: 32,
            },
            animation: AnimationConfig { default_speed: 1.0 },
        }
    }
}
