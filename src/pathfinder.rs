use std::collections::HashMap;
use std::rc::Rc;

use crate::cell::{Cell, CellState};
use crate::field::{Field, Tile};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TestCoord {
    pub x: u16,
    pub y: u16,
}

pub fn pathfinder_a_star(start: Tile, end: Tile, field: &mut Field) -> Option<Vec<TestCoord>> {
    let mut reachable_cells: Vec<Tile> = Vec::new();
    let mut visited_cells: Vec<Tile> = Vec::new();
    let mut ancestral_cells: HashMap<TestCoord, TestCoord> = HashMap::new();

    start.borrow_mut().cell_state = CellState::Visited;
    reachable_cells.push(start.clone());

    println!("Pathfinder start!");

    while !reachable_cells.is_empty() {
        // std::thread::sleep(Duration::from_millis(10));
        if let Some(current_cell) = choose_cell(&reachable_cells) {
            println!(
                "Cell: x:{} y:{}",
                current_cell.borrow().cell_coordinates.x,
                current_cell.borrow().cell_coordinates.y
            );

            if current_cell == end {
                //TODO: rewrite TestCoord to Tile and fix runtime bug!!!
                let mut cell = TestCoord {
                    x: end.borrow().cell_coordinates.x,
                    y: end.borrow().cell_coordinates.y,
                };
                let mut path: Vec<TestCoord> = Vec::new();

                while let Some(parent) = ancestral_cells.get(&cell) {
                    path.push(cell.to_owned());
                    cell = TestCoord {
                        x: parent.x,
                        y: parent.y,
                    };
                }
                path.push(TestCoord {
                    x: start.borrow().cell_coordinates.x,
                    y: start.borrow().cell_coordinates.y,
                });
                path.reverse();

                println!("OK!");
                return Some(path);
            }

            let neighbors = current_cell.borrow_mut().check_neighbors(field);
            for neighbor_cell in neighbors {
                if visited_cells.contains(&neighbor_cell) {
                    continue;
                }

                neighbor_cell.borrow_mut().cell_state = CellState::Visited;
                reachable_cells.remove(0);
                reachable_cells.push(neighbor_cell.to_owned());
                visited_cells.push(neighbor_cell.to_owned());
                let a = TestCoord {
                    x: neighbor_cell.borrow().cell_coordinates.x,
                    y: neighbor_cell.borrow().cell_coordinates.y,
                };
                let b = TestCoord {
                    x: current_cell.borrow().cell_coordinates.x,
                    y: current_cell.borrow().cell_coordinates.y,
                };
                ancestral_cells.insert(a, b);
            }
        }
    }

    None
}

fn build_path() -> Option<Vec<Cell>> {
    Some(vec![])
}

fn choose_cell(reachable_cells: &Vec<Tile>) -> Option<Tile> {
    let rand = rand::random::<u16>() as usize;
    match reachable_cells.get(rand) {
        None => None,
        Some(s) => Some(Rc::clone(s)),
    }
}
