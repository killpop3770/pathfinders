use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::cell::{Cell, CellState};
use crate::field::{Field, Tile};

pub fn pathfinder_a_star(start: Tile, end: Tile, field: &mut Field) -> Option<Vec<Tile>> {
    let mut reachable_cells: Vec<Tile> = Vec::new();
    let mut visited_cells: Vec<Tile> = Vec::new();
    let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();

    start.borrow_mut().cell_state = CellState::Visited;
    reachable_cells.push(start.clone());

    println!("Pathfinder start!");

    while !reachable_cells.is_empty() {
        if let Some(current_cell) = choose_cell(&reachable_cells) {
            println!(
                "Cell: x:{} y:{}",
                current_cell.borrow().cell_coordinates.x,
                current_cell.borrow().cell_coordinates.y
            );

            if current_cell == end {
                //TODO: fix runtime bug ! check unreachable???
                //TODO: decompose render function! render by step of this loop !!
                let mut cell = end;
                let mut path: Vec<Tile> = Vec::new();

                while let Some(parent) = ancestral_cells.get(&cell) {
                    path.push(cell.to_owned());
                    cell = parent.to_owned();
                }
                path.push(start);
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
                reachable_cells.push(neighbor_cell.to_owned());
                visited_cells.push(neighbor_cell.to_owned());
                ancestral_cells.insert(neighbor_cell.to_owned(), current_cell.to_owned());
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
        Some(tile) => Some(Tile::new(Rc::clone(tile.deref()))),
    }
}
