use std::cell::RefCell;
use std::rc::Rc;

use crate::field::{Field, Tile};

#[derive(PartialEq, Debug, Hash, Eq)]
pub struct Cell {
    pub cell_state: CellState,
    pub cell_coordinates: CellCoordinates,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell {
        Cell {
            cell_state: CellState::Empty,
            cell_coordinates: CellCoordinates { x, y },
        }
    }

    // 8-neighbors search algorithm
    pub fn check_neighbors(&self, field: &mut Field) -> Vec<Tile> {
        let main_x = self.cell_coordinates.x as i16;
        let main_y = self.cell_coordinates.y as i16;

        let mut neighbors: Vec<Tile> = Vec::new();

        for side in 0..4 {
            for step in 0..2 {
                if side == 0 {
                    let (x, y) = (main_x + 1, main_y - step);
                    if field.is_valid_coordinate(x, y) && field.is_valid_to_path(x, y) {
                        neighbors.push(make_cell_visited(x, y, field));
                    }
                } else if side == 1 {
                    let (x, y) = (main_x - step, main_y - 1);
                    if field.is_valid_coordinate(x, y) && field.is_valid_to_path(x, y) {
                        neighbors.push(make_cell_visited(x, y, field));
                    }
                } else if side == 2 {
                    let (x, y) = (main_x - 1, main_y + step);
                    if field.is_valid_coordinate(x, y) && field.is_valid_to_path(x, y) {
                        neighbors.push(make_cell_visited(x, y, field));
                    }
                } else if side == 3 {
                    let (x, y) = (main_x + step, main_y + 1);
                    if field.is_valid_coordinate(x, y) && field.is_valid_to_path(x, y) {
                        neighbors.push(make_cell_visited(x, y, field));
                    }
                }
            }
        }

        fn make_cell_visited(x: i16, y: i16, field: &mut Field) -> Tile {
            let mut current_cell_ref = field.get_cell(x as u16, y as u16);
            let mut cell = current_cell_ref.borrow_mut();
            cell.cell_state = CellState::Visited;
            Rc::clone(&current_cell_ref)
        }

        println!("Neghbors: {}", neighbors.len());

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
}

#[derive(PartialEq, Debug, Eq, Hash)]
pub enum CellState {
    Blocked, //obstacles -> Black?
    Visited, //visited cells -> Red 0.5 alpha
    Chosen,  //chosen path -> Green 0.5 alpha
    Empty,   //empty cells -> Gray
}

#[derive(PartialEq, Debug, Hash, Eq)]
pub struct CellCoordinates {
    pub x: u16,
    pub y: u16,
}
