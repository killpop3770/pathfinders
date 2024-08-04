use crate::state::SharedState;
pub mod breadth_first_search;
pub mod depth_first_search;
pub mod greedy_best_first_search;

pub trait Algorithm {
    fn search(&self, state: SharedState);
}
