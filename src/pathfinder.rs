use std::collections::HashMap;

use crate::cell::CellState;
use crate::field::{Field, Tile};

pub fn pathfinder_a_star(start: Tile, end: Tile, field: &mut Field) -> Option<Vec<Tile>> {
    let mut reachable_cells: Vec<Tile> = Vec::new();
    let mut visited_cells: Vec<Tile> = Vec::new();
    let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();

    start.borrow_mut().state = CellState::Visited;
    reachable_cells.push(start.clone());

    println!("Pathfinder start!");

    while !reachable_cells.is_empty() {
        if let Some(current_cell) = choose_cell(&reachable_cells) {
            println!(
                "Cell: x:{} y:{}",
                current_cell.borrow().coordinates.x,
                current_cell.borrow().coordinates.y
            );

            if current_cell == end {
                //TODO: fix runtime bug ! check unreachable???
                //TODO: decompose render function! render by step of this loop !!
                let mut cell = end;
                let mut path: Vec<Tile> = Vec::new();

                while let Some(parent) = ancestral_cells.get(&cell) {
                    path.push(Tile::new(&cell));
                    cell = Tile::new(&parent);
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

                neighbor_cell.borrow_mut().state = CellState::Visited;
                reachable_cells.push(Tile::new(&neighbor_cell));
                visited_cells.push(Tile::new(&neighbor_cell));
                ancestral_cells.insert(Tile::new(&neighbor_cell), Tile::new(&current_cell));
            }
        }
    }

    None
}

// fn build_path() -> Option<Vec<Cell>> {
//     Some(vec![])
// }

fn choose_cell(reachable_cells: &Vec<Tile>) -> Option<Tile> {
    let rand = rand::random::<u16>() as usize;
    match reachable_cells.get(rand) {
        None => None,
        Some(tile) => Some(Tile::new(&tile)),
    }
}
