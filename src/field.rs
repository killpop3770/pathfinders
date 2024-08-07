use std::sync::{Arc, Mutex};

use rand::Rng;

use crate::cell::{Cell, CellState, Tile};

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
    fn is_valid_coordinates(&self, target_x: i16, target_y: i16) -> bool {
        let acceptable = 0..self.cells.len();
        acceptable.contains(&(target_x as usize)) && acceptable.contains(&(target_y as usize))
    }

    //Valid cell to path is cell with Empty type
    fn is_valid_to_path(&self, target_x: i16, target_y: i16) -> bool {
        return match self
            .get_cell(target_x as u16, target_y as u16)
            .lock()
            .unwrap()
            .get_state()
        {
            CellState::Blocked => false,
            _ => true,
        };
    }

    //Create blocks on a field
    pub fn make_noise(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..(self.cells.len().pow(2) as f64 * 0.25).abs() as usize {
            let pos_x = rng.gen_range(0..self.cells.len() as u16);
            let pos_y = rng.gen_range(0..self.cells.len() as u16);
            self.get_cell(pos_x, pos_y)
                .get()
                .set_state(CellState::Blocked);
        }
    }

    pub fn set_prices(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.cells.len() - 1 {
            for y in 0..self.cells.len() - 1 {
                let tile = self.get_cell(x as u16, y as u16);
                if *tile.get().get_state() != CellState::Blocked {
                    tile.get().cost = rng.gen_range(0..20);
                }
            }
        }
    }

    // 4/8-neighbors search algorithm
    pub fn check_cell_neighbors(&mut self, cell: Tile) -> Vec<Tile> {
        let main_x = cell.get().coordinates.x as i16;
        let main_y = cell.get().coordinates.y as i16;
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
                if self.is_valid_coordinates(x, y) && self.is_valid_to_path(x, y) {
                    neighbors.push(self.make_cell_visited(x, y));
                }
            }
        }

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
        current_cell_ref.get().set_state(CellState::Visited);
        current_cell_ref
    }
}
