use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::algorithms::{
    heuristic_manhattan, init_search, reconstruct_and_draw_path, Algorithm, PriorityCell,
};
use crate::domain::cell::AlgoState;
use crate::state::shared_state::SharedState;
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
pub struct GBFS {
    state: SharedState,
}

impl GBFS {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl Algorithm for GBFS {
    fn search(&self) {
        log::info!("GBFS: start");
        let t1 = Instant::now();

        let speed = self.state.speed();
        let size = init_search(&self.state);
        let end_pos = (size - 1, size - 1);

        let mut queue: BinaryHeap<Reverse<PriorityCell>> = BinaryHeap::new();
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        {
            let binding = &mut self.state.lock();
            binding.field_mut().mark_visited(0, 0);
            let h = heuristic_manhattan(0, 0, end_pos.0, end_pos.1);
            queue.push(Reverse(PriorityCell {
                x: 0,
                y: 0,
                cost: h,
            }));
        }

        while let Some(Reverse(current)) = queue.pop() {
            let (x, y) = (current.x, current.y);

            if self.state.should_stop() {
                log::info!("GBFS: stopped by user");
                return;
            }

            self.state.wait(25.0, speed);

            if (x, y) == end_pos {
                log::info!("GBFS: reached end at ({}, {})", x, y);
                log::info!("GBFS: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, (x, y), (0, 0));
                return;
            }

            let neighbors = {
                let binding = &self.state.lock();
                binding.field().get_neighbors(x, y)
            };

            let mut is_end = false;
            let mut end_pos_found = (0, 0);

            {
                let mut guard = self.state.lock();
                let field = guard.field_mut();

                for (nx, ny) in neighbors {
                    if let Some(cell) = field.get_mut(nx, ny) {
                        if cell.is_walkable() {
                            cell.set_algo_state(AlgoState::Visited);
                            visited.insert((nx, ny), (x, y));

                            if cell.is_end() {
                                is_end = true;
                                end_pos_found = (nx, ny);
                                break;
                            }

                            let h = heuristic_manhattan(nx, ny, end_pos.0, end_pos.1);
                            queue.push(Reverse(PriorityCell {
                                x: nx,
                                y: ny,
                                cost: h,
                            }));
                        }
                    }
                }
            }

            if is_end {
                log::info!("GBFS: found end at ({}, {})", end_pos_found.0, end_pos_found.1);
                log::info!("GBFS: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, end_pos_found, (0, 0));
                return;
            }
        }

        log::warn!("GBFS: no path found");
    }
}
