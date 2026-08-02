mod a_star;
mod breadth_first_search;
mod depth_first_search;
mod dijkstra;
mod greedy_best_first_search;

use std::{cmp::Ordering, collections::HashMap};

use crate::{domain::cell::VisualState, state::shared_state::SharedState};

pub use a_star::AStar;
pub use breadth_first_search::BFS;
pub use depth_first_search::DFS;
pub use dijkstra::Dijkstra;
pub use greedy_best_first_search::GBFS;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AlgorithmType {
    BFS,
    DFS,
    GBFS,
    Dijkstra,
    AStar,
}

pub trait Algorithm {
    fn search(&self);

    fn uses_cell_cost(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityCell {
    pub x: usize,
    pub y: usize,
    pub cost: i32,
}

impl Ord for PriorityCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PriorityCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn heuristic_manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}

pub fn init_search(state: &SharedState) -> usize {
    let size = {
        let binding = state.lock();
        binding.field().get_size()
    };

    let mut binding = state.lock();
    let field = binding.field_mut();

    field.reset();
    field.clear_cell(0, 0);
    field.clear_cell(size - 1, size - 1);
    field.set_start(0, 0);
    field.set_end(size - 1, size - 1);

    size
}

pub fn reconstruct_and_draw_path(
    state: &SharedState,
    visited: &HashMap<(usize, usize), (usize, usize)>,
    end_pos: (usize, usize),
    start_pos: (usize, usize),
) {
    let mut path = Vec::new();
    let mut current = end_pos;

    while let Some(&parent) = visited.get(&current) {
        path.push(current);
        if current == start_pos {
            break;
        }
        current = parent;
    }
    path.reverse();

    let mut binding = state.lock();
    let field = binding.field_mut();
    for &(px, py) in &path {
        if let Some(cell) = field.get_mut(px, py) {
            if !cell.is_start() && !cell.is_end() {
                cell.set_visual_state(VisualState::Path);
            }
        }
    }
}
