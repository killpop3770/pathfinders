use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::cell::{Cell, CellState};

pub const EMPTY_FIELD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const EMPTY_CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const CHOSEN_CELL_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLOCKED_CELL_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const VISITED_CELL_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.5];

pub type Tile = Rc<RefCell<Cell>>;

pub struct Field {
    pub cells: Vec<Vec<Tile>>,
}

impl Field {
    pub fn new(cells_number: u16) -> Field {
        Field {
            cells: (0..cells_number)
                .map(|x| {
                    (0..cells_number)
                        .map(|y| Rc::new(RefCell::new(Cell::new(x, y))))
                        .collect()
                })
                .collect::<Vec<Vec<Tile>>>(),
        }
    }
    pub fn get_cell(&self, x: u16, y: u16) -> Tile {
        Rc::clone(&self.cells[x as usize][y as usize])
    }

    //Check position by bounds
    pub fn is_valid_coordinate(&self, target_x: i16, target_y: i16) -> bool {
        let acceptable = 0..self.cells.len();
        acceptable.contains(&(target_x as usize)) && acceptable.contains(&(target_y as usize))
    }

    //Valid cell to path is cell with Empty type
    pub fn is_valid_to_path(&self, target_x: i16, target_y: i16) -> bool {
        self.get_cell(target_x as u16, target_y as u16)
            .borrow_mut()
            .cell_state
            == CellState::Empty
    }

    //Create blocks on a field/map
    pub fn make_noise(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let pos_x = rng.gen_range(0..self.cells.len() as u16);
            let pos_y = rng.gen_range(0..self.cells.len() as u16);
            self.get_cell(pos_x, pos_y).borrow_mut().cell_state = CellState::Blocked;
        }
    }
}
