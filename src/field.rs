use std::hash::Hash;
use std::ops::Mul;
use std::sync::{Arc, Mutex};

use rand::Rng;

use crate::cell::{Cell, CellState, Tile};

pub const EMPTY_FIELD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const EMPTY_CELL_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const CHOSEN_CELL_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLOCKED_CELL_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const VISITED_CELL_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.5];

//TODO:
pub const TEST_CELL_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct Field {
    pub cells: Vec<Vec<Tile>>,
}

impl Field {
    pub fn new(cells_number: u16) -> Field {
        Field {
            cells: (0..cells_number)
                .map(|x| {
                    (0..cells_number)
                        .map(|y| Tile(Arc::new(Mutex::new(Cell::new(x, y)))))
                        .collect()
                })
                .collect::<Vec<Vec<Tile>>>(),
        }
    }
    pub fn get_cell(&self, x: u16, y: u16) -> Tile {
        self.cells[x as usize][y as usize].clone()
    }

    //Check position by bounds
    pub fn is_valid_coordinate(&self, target_x: i16, target_y: i16) -> bool {
        let acceptable = 0..self.cells.len();
        acceptable.contains(&(target_x as usize)) && acceptable.contains(&(target_y as usize))
    }

    //Valid cell to path is cell with Empty type
    pub fn is_valid_to_path(&self, target_x: i16, target_y: i16) -> bool {
        self.get_cell(target_x as u16, target_y as u16)
            .lock()
            .unwrap()
            .state
            == CellState::Empty
            || self
                .get_cell(target_x as u16, target_y as u16)
                .lock()
                .unwrap()
                .state
                == CellState::TEST
    }

    //Create blocks on a field
    pub fn make_noise(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..(self.cells.len().pow(2) as f64 * 0.25).abs() as usize {
            let pos_x = rng.gen_range(0..self.cells.len() as u16);
            let pos_y = rng.gen_range(0..self.cells.len() as u16);
            self.get_cell(pos_x, pos_y).lock().unwrap().state = CellState::Blocked;
        }
    }

    // 4/8-neighbors search algorithm
    pub fn check_cell_neighbors(&mut self, cell: Tile) -> Vec<Tile> {
        let main_x = cell.lock().unwrap().coordinates.x as i16;
        let main_y = cell.lock().unwrap().coordinates.y as i16;
        let mut neighbors: Vec<Tile> = Vec::new();

        //nest cell
        for side in 0..4 {
            for step in 0..1 {
                let (x, y) = match side {
                    0 => (main_x + 1, main_y - step),
                    1 => (main_x - step, main_y - 1),
                    2 => (main_x - 1, main_y + step),
                    3 => (main_x + step, main_y + 1),
                    _ => (0, 0),
                };
                // println!("side: {side}, step: {step}, x: {x}, y: {y}");
                if self.is_valid_coordinate(x, y) && self.is_valid_to_path(x, y) {
                    neighbors.push(self.make_cell_visited(x, y));
                }
            }
        }

        // println!("Neighbors: {}", neighbors.len());
        return neighbors;

        // It's ok at 3:2
        // It's ok at 3:1
        // It's ok at 2:1
        // It's ok at 1:1
        // It's ok at 1:2
        // It's ok at 1:3
        // It's ok at 2:3
        // It's ok at 3:3

        //  1;1   2;1    3;1
        //  1;2   2;2    3;2
        //  1;3   2;3    3;3
    }

    fn make_cell_visited(&self, x: i16, y: i16) -> Tile {
        let current_cell_ref = self.get_cell(x as u16, y as u16);
        current_cell_ref.lock().unwrap().state = CellState::Visited;
        current_cell_ref
    }
}
