use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::algorithms::{
    heuristic_manhattan, init_search, reconstruct_and_draw_path, Algorithm, PriorityCell,
};
use crate::domain::cell::AlgoState;
use crate::state::shared_state::SharedState;
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
pub struct AStar {
    state: SharedState,
}

impl AStar {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl Algorithm for AStar {
    fn search(&self) {
        log::info!("A*: start");
        let t1 = Instant::now();

        let speed = self.state.speed();
        let size = init_search(&self.state);
        let end_pos = (size - 1, size - 1);

        let mut queue: BinaryHeap<Reverse<PriorityCell>> = BinaryHeap::new();
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut cost_so_far: HashMap<(usize, usize), u16> = HashMap::new();

        {
            let binding = &mut self.state.lock();
            let start_cost = binding.field().cell(0, 0).cost;
            binding.field_mut().mark_visited(0, 0);
            cost_so_far.insert((0, 0), start_cost);
            let h = heuristic_manhattan(0, 0, end_pos.0, end_pos.1);
            queue.push(Reverse(PriorityCell {
                x: 0,
                y: 0,
                cost: (start_cost as i32) + h,
            }));
        }

        while let Some(Reverse(current)) = queue.pop() {
            let (x, y) = (current.x, current.y);

            if self.state.should_stop() {
                log::info!("A*: stopped by user");
                return;
            }

            self.state.wait(25.0, speed);

            if (x, y) == end_pos {
                log::info!("A*: reached end at ({}, {})", x, y);
                log::info!("A*: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, (x, y), (0, 0));
                return;
            }

            let neighbors = {
                let binding = &self.state.lock();
                binding.field().get_neighbors(x, y)
            };

            let current_g = cost_so_far[&(x, y)];

            let mut is_end = false;
            let mut end_pos_found = (0, 0);

            {
                let mut guard = self.state.lock();
                let field = guard.field_mut();

                for (nx, ny) in neighbors {
                    if let Some(cell) = field.get_mut(nx, ny) {
                        if cell.is_walkable() {
                            let new_g = current_g + cell.cost;
                            let is_better = cost_so_far
                                .get(&(nx, ny))
                                .is_none_or(|&old_g| new_g < old_g);

                            if is_better {
                                cost_so_far.insert((nx, ny), new_g);
                                visited.insert((nx, ny), (x, y));
                                cell.set_algo_state(AlgoState::Visited);

                                if cell.is_end() {
                                    is_end = true;
                                    end_pos_found = (nx, ny);
                                    break;
                                }

                                let h = heuristic_manhattan(nx, ny, end_pos.0, end_pos.1);
                                queue.push(Reverse(PriorityCell {
                                    x: nx,
                                    y: ny,
                                    cost: (new_g as i32) + h,
                                }));
                            }
                        }
                    }
                }
            }

            if is_end {
                log::info!("A*: found end at ({}, {})", end_pos_found.0, end_pos_found.1);
                log::info!("A*: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, end_pos_found, (0, 0));
                return;
            }
        }

        log::warn!("A*: no path found");
    }

    fn uses_cell_cost(&self) -> bool {
        true
    }
}
