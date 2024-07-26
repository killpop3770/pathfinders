use lazy_static::lazy_static;
use crate::field::{Cell, CellCoordinates, CellState, Field};

pub fn pathfinder_a_star(start: &mut Cell, end: &mut Cell, field: Field) -> Option<Vec<Cell>> {
    let mut reachable_cells: Vec<&Cell> = Vec::new();
    let explored_cells: Vec<Cell> = Vec::new();

    start.cell_type = CellState::Visited;
    reachable_cells.push(start);


    while !reachable_cells.is_empty() {
        let current_cell = chose_cell(&reachable_cells);
        // как проверить есть ли у ячейки соседи по бокам и диагонолям?

        if current_cell == end {
            build_path();
        }
    }

    Some(vec![])
}

fn build_path() -> Option<Vec<Cell>> {
    Some(vec![])
}

lazy_static! {
    static ref TEST_ARR: [Cell; 1] = [Cell::new(0,0)];
}
fn chose_cell<'a>(reachable_cells: &Vec<&Cell>) -> &'a Cell {
    let ret = &TEST_ARR[0];
    ret
}

pub fn check_neighbors(coord: CellCoordinates, field: &Field) {
    println!("{} {}", coord.x, coord.y);
    let main_x = coord.x as i16;
    let main_y = coord.y as i16;

    for side in 0..4 {
        for step in 0..2 {
            if side == 0 {
                if is_valid_coordinate(main_x + 1) && is_valid_coordinate(main_y - step) {
                    println!("It's ok at {}:{}", main_x + 1, main_y - step)
                }
            } else if side == 1 {
                if is_valid_coordinate(main_x - step) && is_valid_coordinate(main_y - 1) {
                    println!("It's ok at {}:{}", main_x - step, main_y - 1)
                }
            } else if side == 2 {
                if is_valid_coordinate(main_x - 1) && is_valid_coordinate(main_y + step) {
                    println!("It's ok at {}:{}", main_x - 1, main_y + step)
                }
            } else if side == 3 {
                if is_valid_coordinate(main_x + step) && is_valid_coordinate(main_y + 1) {
                    println!("It's ok at {}:{}", main_x + step, main_y + 1)
                }
            }
        }
    }

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

fn is_valid_coordinate(target_pos: i16) -> bool {
    target_pos >= 0 && target_pos < 20
}