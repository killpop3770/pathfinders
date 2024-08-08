use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::algorithms::{colorize_path, Algorithm, PriorityCell};
use crate::cell::{CellState, Tile};
use crate::state::SharedState;

pub struct Dijkstra(pub Arc<AtomicBool>);

impl Algorithm for Dijkstra {
    fn search(&self, state: SharedState) {
        let mut reachable_cells: BinaryHeap<Reverse<PriorityCell>> = BinaryHeap::new();
        let mut visited_cells: Vec<Tile> = Vec::new();
        let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();
        let mut cost_so_far: HashMap<Tile, i16> = HashMap::new();

        let start_cell = state.get().field().get_cell(0, 0).clone();
        start_cell.get().set_state(CellState::Start);

        let end_coords_value = (state.get().field().cells.len() - 1) as u16;
        let end_cell = state
            .get()
            .field()
            .get_cell(end_coords_value, end_coords_value)
            .clone();
        end_cell.get().set_state(CellState::End);

        let priority = start_cell.get().cost;
        reachable_cells.push(Reverse(PriorityCell {
            tile: start_cell.clone(),
            cost: priority,
        }));
        visited_cells.push(start_cell.clone());

        let start_cell_cost = start_cell.get().cost;
        cost_so_far.insert(start_cell.clone(), start_cell_cost);

        while let Some(current_cell) = reachable_cells.pop() {
            if self.0.load(Ordering::Relaxed) {
                break;
            }
            state.wait(25.0);
            println!(
                "r {} | v {} | a {}",
                reachable_cells.len(),
                visited_cells.len(),
                ancestral_cells.len()
            );
            let current_cell = current_cell.0.tile.clone();
            current_cell.get().set_state(CellState::Visited);

            if current_cell == end_cell {
                let mut cell = end_cell.clone();
                let mut path: Vec<Tile> = Vec::new();

                while let Some(parent) = ancestral_cells.get(&cell) {
                    path.push(cell.clone());
                    cell = parent.clone();
                }
                path.push(start_cell.clone());
                path.reverse();
                println!("p {}", path.len());
                colorize_path(path);
                break;
            }

            let neighbor_cells = state
                .get()
                .field()
                .check_cell_neighbors(current_cell.clone());
            for neighbor_cell in neighbor_cells {
                if visited_cells.contains(&neighbor_cell) {
                    continue;
                }

                let current_cell_cost = current_cell.get().cost;
                let neighbor_cell_cost = neighbor_cell.get().cost;
                let new_cost = current_cell_cost + neighbor_cell_cost;
                if cost_so_far.get(&neighbor_cell.clone()).is_none()
                    || new_cost < *cost_so_far.get(&neighbor_cell.clone()).unwrap()
                {
                    cost_so_far.insert(neighbor_cell.clone(), new_cost);

                    let priority = new_cost;
                    reachable_cells.push(Reverse(PriorityCell {
                        tile: neighbor_cell.clone(),
                        cost: priority,
                    }));
                    visited_cells.push(neighbor_cell.clone());
                    ancestral_cells.insert(neighbor_cell.clone(), current_cell.clone());
                }
            }
        }
    }
}
