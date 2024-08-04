use crate::algorithms::Algorithm;
use crate::cell::{CellState, Tile};
use crate::state::SharedState;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

pub struct DFS;

impl Algorithm for DFS {
    fn search(&self, state: SharedState) {
        let mut reachable_cells: VecDeque<Tile> = VecDeque::new();
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

        reachable_cells.push_front(start_cell.clone());
        visited_cells.push(start_cell.clone());

        while let Some(current_cell) = reachable_cells.pop_front() {
            std::thread::sleep(Duration::from_millis(1));
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

                reachable_cells.push_front(neighbor_cell.clone());
                visited_cells.push(neighbor_cell.clone());
                ancestral_cells.insert(neighbor_cell.clone(), current_cell.clone());
            }
        }
    }
}

fn colorize_path(path: Vec<Tile>) {
    path.iter()
        .map(|tile| tile.get().set_state(CellState::Chosen))
        .count();
}
