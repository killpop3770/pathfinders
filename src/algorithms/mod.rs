use crate::cell::{CellState, Tile};
use crate::state::SharedState;
use std::cmp::Ordering;
pub mod a_star;
pub mod breadth_first_search;
pub mod depth_first_search;
pub mod dijkstra;
pub mod greedy_best_first_search;

#[derive(Copy, Clone)]
pub enum AlgorithmType {
    BFS,
    DFS,
    GBFS,
    Dijkstra,
    AStar,
}

pub trait Algorithm {
    fn search(&self, state: SharedState);
}

#[derive(Debug)]
pub struct PriorityCell {
    pub tile: Tile,
    pub cost: i16,
}

impl Eq for PriorityCell {}

impl PartialEq<Self> for PriorityCell {
    fn eq(&self, other: &Self) -> bool {
        self.tile == other.tile
    }
}

impl PartialOrd<Self> for PriorityCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for PriorityCell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

pub fn heuristic_factor(a: Tile, b: Tile) -> i16 {
    let ax = a.get().coordinates.x as i16;
    let bx = b.get().coordinates.x as i16;
    let ay = a.get().coordinates.y as i16;
    let by = b.get().coordinates.y as i16;
    return (ax - bx).abs() + (ay - by).abs();
}

pub fn colorize_path(path: Vec<Tile>) {
    path.iter()
        .map(|tile| tile.get().set_state(CellState::Chosen))
        .count();
}
