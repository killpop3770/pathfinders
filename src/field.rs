pub const EMPTY_FIELD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const EMPTY_CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const CHOSEN_CELL_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
pub const BLOCKED_CELL_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct CellCoordinates {
    pub x: u16,
    pub y: u16,
}

pub struct Field {
    pub cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn new(cells_number: u16) -> Field {
        Field {
            cells: (0..cells_number).map(|x| {
                (0..cells_number).map(|y| {
                    Cell::new()
                }).collect()
            }).collect::<Vec<Vec<Cell>>>(),
        }
    }

    pub fn get_cell(&mut self, x: u16, y: u16) -> &mut Cell {
        &mut self.cells[x as usize][y as usize]
    }
}

pub struct Cell {
    pub cell_type: CellType,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            cell_type: CellType::Empty,
        }
    }
}

#[derive(PartialEq)]
pub enum CellType {
    Blocked,
    Chosen,
    Empty,
}