#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgoState {
    Unvisited,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisualState {
    None,
    Start,
    End,
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct Cell {
    cell_type: CellType,
    algorithm_state: AlgoState,
    visual_state: VisualState,
    pub cost: u16,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            cell_type: CellType::Empty,
            cost: 10,
            algorithm_state: AlgoState::Unvisited,
            visual_state: VisualState::None,
        }
    }

    pub fn get_cell_type(&self) -> CellType {
        self.cell_type
    }

    pub fn set_cell_type(&mut self, state: CellType) {
        self.cell_type = state;
    }

    pub fn set_algo_state(&mut self, state: AlgoState) {
        self.algorithm_state = state;
    }

    pub fn get_algo_state(&self) -> AlgoState {
        self.algorithm_state
    }

    pub fn set_visual_state(&mut self, state: VisualState) {
        self.visual_state = state;
    }

    pub fn get_visual_state(&self) -> VisualState {
        self.visual_state
    }

    pub fn is_walkable(&self) -> bool {
        self.cell_type != CellType::Blocked && self.algorithm_state != AlgoState::Visited
    }

    pub fn is_start(&self) -> bool {
        self.visual_state == VisualState::Start
    }

    pub fn is_end(&self) -> bool {
        self.visual_state == VisualState::End
    }
}
