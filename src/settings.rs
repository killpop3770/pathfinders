pub struct Vec2f {
    pub raw_x: f64,
    pub raw_y: f64,
}

pub struct Settings {
    pub window_size: Vec2f,
    pub cell_size: Vec2f,
    pub cells_number: u16,
}

impl Settings {
    pub fn new(cell_size: u16, cells_number: u16) -> Settings {
        Settings {
            window_size: Vec2f {
                raw_x: (cell_size * cells_number) as f64,
                raw_y: (cell_size * cells_number) as f64,
            },
            cell_size: Vec2f { raw_x: cell_size as f64, raw_y: cell_size as f64 },
            cells_number,
        }
    }
}
