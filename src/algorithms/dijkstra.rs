use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::algorithms::{init_search, reconstruct_and_draw_path, Algorithm, PriorityCell};
use crate::domain::cell::AlgoState;
use crate::state::shared_state::SharedState;
use std::time::Instant;

#[allow(clippy::upper_case_acronyms)]
pub struct Dijkstra {
    state: SharedState,
}

impl Dijkstra {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

impl Algorithm for Dijkstra {
    fn search(&self) {
        log::info!("Dijkstra: start");
        let t1 = Instant::now();

        let speed = self.state.speed();
        let size = init_search(&self.state);

        let mut queue: BinaryHeap<Reverse<PriorityCell>> = BinaryHeap::new();
        let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut cost_so_far: HashMap<(usize, usize), u16> = HashMap::new();

        {
            let binding = &mut self.state.lock();
            let start_cost = binding.field().cell(0, 0).cost;
            binding.field_mut().mark_visited(0, 0);
            cost_so_far.insert((0, 0), start_cost);
            queue.push(Reverse(PriorityCell {
                x: 0,
                y: 0,
                cost: start_cost as i32,
            }));
        }

        while let Some(Reverse(current)) = queue.pop() {
            let (x, y) = (current.x, current.y);

            if self.state.should_stop() {
                log::info!("Dijkstra: stopped by user");
                return;
            }

            self.state.wait(25.0, speed);

            if (x, y) == (size - 1, size - 1) {
                log::info!("Dijkstra: reached end at ({}, {})", x, y);
                log::info!("Dijkstra: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, (x, y), (0, 0));
                return;
            }

            let neighbors = {
                let binding = &self.state.lock();
                binding.field().get_neighbors(x, y)
            };

            let current_cost = cost_so_far[&(x, y)];

            let mut is_end = false;
            let mut end_pos = (0, 0);

            {
                let mut guard = self.state.lock();
                let field = guard.field_mut();

                for (nx, ny) in neighbors {
                    if let Some(cell) = field.get_mut(nx, ny) {
                        if cell.is_walkable() {
                            let new_cost = current_cost + cell.cost;
                            let is_better = cost_so_far
                                .get(&(nx, ny))
                                .is_none_or(|&old_cost| new_cost < old_cost);

                            if is_better {
                                cost_so_far.insert((nx, ny), new_cost);
                                visited.insert((nx, ny), (x, y));
                                cell.set_algo_state(AlgoState::Visited);

                                if cell.is_end() {
                                    is_end = true;
                                    end_pos = (nx, ny);
                                    break;
                                }

                                queue.push(Reverse(PriorityCell {
                                    x: nx,
                                    y: ny,
                                    cost: new_cost as i32,
                                }));
                            }
                        }
                    }
                }
            }

            if is_end {
                log::info!("Dijkstra: found end at ({}, {})", end_pos.0, end_pos.1);
                log::info!("Dijkstra: elapsed time: {:?}", Instant::now() - t1);
                reconstruct_and_draw_path(&self.state, &visited, end_pos, (0, 0));
                return;
            }
        }

        log::warn!("Dijkstra: no path found");
    }

    fn uses_cell_cost(&self) -> bool {
        true
    }
}
