use crate::algorithms::{init_search, reconstruct_and_draw_path, Algorithm};
use crate::domain::cell::AlgoState;
use crate::state::shared_state::SharedState;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
pub struct BFS {
    state: SharedState,
}

impl BFS {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl Algorithm for BFS {
    fn search(&self) {
        log::info!("BFS: start");
        let t1 = Instant::now();

        let speed = self.state.speed();
        let size = init_search(&self.state);
        log::info!("BFS: field size = {}", size);

        let mut queue = VecDeque::new();
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        {
            let binding = &mut self.state.lock();
            binding.field_mut().mark_visited(0, 0);
        }
        queue.push_back((0_usize, 0_usize));

        while let Some((x, y)) = queue.pop_front() {
            if self.state.should_stop() {
                log::info!("BFS: finished");
                log::info!("BFS: stopped by user");
                let t2 = Instant::now();
                log::info!("BFS: elapsed time: {:?}", t2 - t1);
                return;
            }

            self.state.wait(25.0, speed);

            let neighbors = {
                let binding = &self.state.lock();
                binding.field().get_neighbors(x, y)
            };

            let mut is_end = false;
            let mut end_pos = (0, 0);

            {
                let guard = &mut self.state.lock();
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

                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            if is_end {
                log::info!("BFS: found end at ({}, {})", end_pos.0, end_pos.1);
                let t2 = Instant::now();
                log::info!("BFS: elapsed time: {:?}", (t2 - t1));
                reconstruct_and_draw_path(&self.state, &visited, (end_pos.0, end_pos.1), (0, 0));
                return;
            }
        }
    }
}
