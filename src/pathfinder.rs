use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use crate::cell::{CellState, Tile};
use crate::field::Field;
use crate::state::SharedState;

pub trait Algorithm {
    fn search(start: Tile, end: Tile, field: &mut Field);
}

pub struct PathfinderAStar;

impl PathfinderAStar {
    pub fn search(&self, state: SharedState) {
        let mut reachable_cells: Vec<Tile> = Vec::new();
        let mut visited_cells: Vec<Tile> = Vec::new();
        let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();

        let start = state.get().field.get_cell(0, 0).clone();

        //TODO:
        let test_bound_value = (state.get().field.cells.len() - 1) as u16;
        let end = state
            .get()
            .field
            .get_cell(test_bound_value, test_bound_value)
            .clone();
        end.lock().unwrap().state = CellState::TEST;

        start.lock().unwrap().state = CellState::Visited;
        reachable_cells.push(start.clone());

        while !reachable_cells.is_empty() {
            println!("=========BEGIN==========");
            std::thread::sleep(Duration::from_nanos(700));
            if let Some(current_cell) = choose_cell(&reachable_cells) {
                print!("Cell: x:{} ", current_cell.lock().unwrap().coordinates.x);
                println!("Cell: y:{}", current_cell.lock().unwrap().coordinates.y);
                println!("Hashmap : {}", ancestral_cells.len());
                println!("REACH : {}", reachable_cells.len());
                println!("VISIT : {}", visited_cells.len());

                if current_cell == end {
                    let mut cell = end.clone();
                    let mut path: Vec<Tile> = Vec::new();

                    while let Some(parent) = ancestral_cells.get(&cell) {
                        path.push(cell.clone());
                        cell = parent.clone();
                    }
                    path.push(start.clone());
                    path.reverse();
                    colorize_path(path);
                    break;
                }

                let neighbors = state.get().field.check_cell_neighbors(current_cell.clone());
                for neighbor_cell in neighbors {
                    if visited_cells.contains(&neighbor_cell) {
                        println!("visited"); //TODO:
                        continue;
                    }

                    neighbor_cell.lock().unwrap().state = CellState::Visited;
                    reachable_cells.push(neighbor_cell.clone());
                    visited_cells.push(neighbor_cell.clone());
                    ancestral_cells.insert(neighbor_cell.clone(), current_cell.clone());
                }
                println!("==========END===========");
            }
        }
    }
}

fn colorize_path(arr: Vec<Tile>) {
    println!("BEGIN COLORIZE PATH");
    arr.iter()
        .map(|x| x.lock().unwrap().state = CellState::Chosen)
        .count();
}

fn choose_cell(reachable_cells: &Vec<Tile>) -> Option<Tile> {
    let rand = thread_rng().gen_range(0..reachable_cells.len());
    match reachable_cells.get(rand) {
        None => None,
        Some(tile) => Some(tile.clone()),
    }
}
