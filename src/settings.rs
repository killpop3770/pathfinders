pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

pub struct Settings {
    pub window_size: Vec2f,
    pub cell_size: Vec2f,
    pub cells_number: u16,
}

impl Settings {
    pub fn new(window_size: Vec2f, cell_size: Vec2f, cells_number: u16) -> Settings {
        Settings {
            window_size,
            cell_size,
            cells_number,
        }
    }
}