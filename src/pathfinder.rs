use std::collections::HashMap;
use std::time::Duration;

use rand::{Rng, thread_rng};

use crate::cell::{CellState, Tile};
use crate::state::SharedState;

pub trait Algorithm {
    fn search(&self, state: SharedState);
}

pub struct PathfinderAStar;

impl Algorithm for PathfinderAStar {
    fn search(&self, state: SharedState) {
        let mut reachable_cells: Vec<Tile> = Vec::new();
        let mut visited_cells: Vec<Tile> = Vec::new();
        let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();

        let start_cell = state.get().field().get_cell(0, 0).clone();
        start_cell.get().set_state(CellState::Start);
        let end_coords_value = (state.get().field().cells.len() - 1) as u16;
        let end_cell = state.get().field().get_cell(end_coords_value, end_coords_value).clone();
        end_cell.get().set_state(CellState::End);

        reachable_cells.push(start_cell.clone());

        while !reachable_cells.is_empty() {
            std::thread::sleep(Duration::from_nanos(800));
            if let Some(current_cell) = choose_cell(&reachable_cells) {
                current_cell.get().set_state(CellState::Visited);
                if current_cell == end_cell {
                    println!("OK!");
                    let mut cell = end_cell.clone();
                    let mut path: Vec<Tile> = Vec::new();

                    while let Some(parent) = ancestral_cells.get(&cell) {
                        path.push(cell.clone());
                        cell = parent.clone();
                    }
                    path.push(start_cell.clone());
                    path.reverse();
                    colorize_path(path);
                    break;
                }

                let neighbors = state.get().field().check_cell_neighbors(current_cell.clone());
                for neighbor_cell in neighbors {
                    if visited_cells.contains(&neighbor_cell) {
                        continue;
                    }

                    visited_cells.push(current_cell.clone());
                    reachable_cells.push(neighbor_cell.clone());
                    ancestral_cells.insert(neighbor_cell.clone(), current_cell.clone());
                }
            }
        }
    }
}

fn colorize_path(arr: Vec<Tile>) {
    arr.iter()
        .map(|x| x.get().set_state(CellState::Chosen))
        .count();
}

fn choose_cell(reachable_cells: &Vec<Tile>) -> Option<Tile> {
    let rand = thread_rng().gen_range(0..reachable_cells.len());
    match reachable_cells.get(rand) {
        None => None,
        Some(tile) => Some(tile.clone()),
    }
}
