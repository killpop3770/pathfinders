pub const EMPTY_FIELD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const EMPTY_CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const CHOSEN_CELL_COLOR: [f32; 4] = [0.0, 0.1, 0.0, 0.5];
pub const BLOCKED_CELL_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const VISITED_CELL_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.5];

#[derive(PartialEq, Debug)]
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
                    Cell::new(x, y)
                }).collect()
            }).collect::<Vec<Vec<Cell>>>(),
        }
    }

    pub fn get_cell(&mut self, x: u16, y: u16) -> &mut Cell {
        &mut self.cells[x as usize][y as usize]
    }
}

#[derive(PartialEq)]
pub struct Cell {
    pub cell_type: CellState,
    pub cell_coordinates: CellCoordinates,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell {
        Cell {
            cell_type: CellState::Empty,
            cell_coordinates: CellCoordinates { x, y },
        }
    }
}

#[derive(PartialEq)]
pub enum CellState {
    Blocked, //obstacles -> Black?
    Visited, //visited cells -> Red 0.5 alpha
    Chosen, //chosen path -> Green 0.5 alpha
    Empty, //empty cells -> Gray
}