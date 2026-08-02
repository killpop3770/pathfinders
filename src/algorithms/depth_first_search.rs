use std::collections::{HashMap, VecDeque};

use crate::algorithms::{init_search, reconstruct_and_draw_path, Algorithm};
use crate::domain::cell::AlgoState;
use crate::state::shared_state::SharedState;
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
pub struct DFS {
    state: SharedState,
}

impl DFS {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl Algorithm for DFS {
    fn search(&self) {
        log::info!("DFS: start");
        let t1 = Instant::now();

        let speed = self.state.speed();
        let size = init_search(&self.state);

        let mut stack = VecDeque::new();
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        {
            let binding = &mut self.state.lock();
            binding.field_mut().mark_visited(0, 0);
        }
        stack.push_back((0_usize, 0_usize));

        while let Some((x, y)) = stack.pop_back() {
            // LIFO
            if self.state.should_stop() {
                log::info!("DFS: stopped by user");
                return;
            }

            self.state.wait(25.0, speed);

            if (x, y) == (size - 1, size - 1) {
                log::info!("DFS: reached end at ({}, {})", x, y);
                log::info!("DFS: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, (x, y), (0, 0));
                return;
            }

            let neighbors = {
                let binding = &self.state.lock();
                binding.field().get_neighbors(x, y)
            };

            let mut is_end = false;
            let mut end_pos = (0, 0);

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
                                end_pos = (nx, ny);
                                break;
                            }

                            stack.push_back((nx, ny));
                        }
                    }
                }
            }

            if is_end {
                log::info!("DFS: found end at ({}, {})", end_pos.0, end_pos.1);
                log::info!("DFS: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, end_pos, (0, 0));
                return;
            }
        }

        log::warn!("DFS: no path found");
    }
}
