use crate::domain::cell::{AlgoState, Cell, CellType, VisualState};
use rand::Rng;

#[derive(Clone)]
pub struct Field {
    cells: Vec<Cell>,
    size: usize,
}

impl Field {
    pub fn new(size: u16) -> Self {
        let size = size as usize;
        let total_cells = size * size;

        // Initialize flat vec (minimal allocator calls)
        let cells = vec![Cell::new(); total_cells];
        Self { cells, size }
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.size && y < self.size {
            Some(&self.cells[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.size && y < self.size {
            let idx = self.index(x, y);
            Some(&mut self.cells[idx])
        } else {
            None
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        self.get(x, y).expect("Cell coordinates out of bounds")
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn is_valid_coords(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.size && (y as usize) < self.size
    }

    fn is_passable(&self, x: isize, y: isize) -> bool {
        self.is_valid_coords(x, y)
            && self.get(x as usize, y as usize).unwrap().get_cell_type() != CellType::Blocked
    }

    pub fn make_noise(&mut self) {
        let mut rng = rand::thread_rng();
        let total = self.size * self.size;
        let obstacles = (total as f64 * 0.25) as usize;

        for _ in 0..obstacles {
            let x = rng.gen_range(0..self.size);
            let y = rng.gen_range(0..self.size);
            if let Some(cell) = self.get_mut(x, y) {
                cell.set_cell_type(CellType::Blocked);
            }
        }
    }

    pub fn set_prices(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in &mut self.cells {
            if cell.get_cell_type() != CellType::Blocked {
                cell.cost = rng.gen_range(1..=20);
            }
        }
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut neighbors = Vec::with_capacity(4);

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if self.is_passable(nx, ny) {
                neighbors.push((nx as usize, ny as usize));
            }
        }
        neighbors
    }

    pub fn mark_visited(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.set_algo_state(AlgoState::Visited);
        }
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.set_visual_state(VisualState::Start);
        }
    }

    pub fn set_end(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.set_visual_state(VisualState::End);
        }
    }

    pub fn clear_cell(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.set_cell_type(CellType::Empty);
            cell.set_algo_state(AlgoState::Unvisited);
            cell.set_visual_state(VisualState::None);
            cell.cost = 10; // default cost
        }
    }

    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            cell.set_algo_state(AlgoState::Unvisited);
            cell.set_visual_state(VisualState::None);
        }
    }
}
