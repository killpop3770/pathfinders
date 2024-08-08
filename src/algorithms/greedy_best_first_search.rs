use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::algorithms::{colorize_path, heuristic_factor, Algorithm, PriorityCell};
use crate::cell::{CellState, Tile};
use crate::state::SharedState;

pub struct GBFS(pub Arc<AtomicBool>);

impl Algorithm for GBFS {
    fn search(&self, state: SharedState) {
        let mut reachable_cells: BinaryHeap<Reverse<PriorityCell>> = BinaryHeap::new();
        let mut visited_cells: Vec<Tile> = Vec::new();
        let mut ancestral_cells: HashMap<Tile, Tile> = HashMap::new();

        let start_cell = state.get().field().get_cell(0, 0).clone();
        start_cell.get().set_state(CellState::Start);

        let end_coords_value = (state.get().field().cells.len() - 1) as u16;
        let end_cell = state
            .get()
            .field()
            .get_cell(end_coords_value, end_coords_value)
            .clone();
        end_cell.get().set_state(CellState::End);

        let priority = heuristic_factor(start_cell.clone(), end_cell.clone());
        reachable_cells.push(Reverse(PriorityCell {
            tile: start_cell.clone(),
            cost: priority,
        }));
        visited_cells.push(start_cell.clone());

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

                let priority = heuristic_factor(neighbor_cell.clone(), end_cell.clone());
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
