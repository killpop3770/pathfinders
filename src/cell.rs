use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Tile(pub Arc<Mutex<Cell>>);

impl Tile {
    pub fn get(&self) -> MutexGuard<'_, Cell> {
        self.lock().unwrap()
    }
}

impl Eq for Tile {}

//TODO:
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        let x1 = self.get().coordinates.x;
        let x2 = other.get().coordinates.x;
        let y1 = self.get().coordinates.y;
        let y2 = other.get().coordinates.y;
        x1 == x2 && y1 == y2
        // self.get().coordinates == other.get().coordinates //absence of atomicity condition?
    }
}

impl Deref for Tile {
    type Target = Arc<Mutex<Cell>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile(Arc::clone(self))
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().state.hash(state);
        self.get().coordinates.hash(state);
    }
}

#[derive(Debug, Hash, Eq)]
pub struct Cell {
    state: CellState,
    pub coordinates: CellCoordinates,
    pub name: String,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell {
        Cell {
            state: CellState::Empty,
            coordinates: CellCoordinates { x, y },
            name: format!("CELL_{}_{}", x, y),
        }
    }

    pub fn get_state(&self) -> &CellState {
        &self.state
    }

    pub fn set_state(&mut self, state: CellState) {
        self.state = state;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

//TODO: another way to colorize start/end cells
#[derive(PartialEq, Debug, Eq, Hash)]
pub enum CellState {
    Blocked, //obstacles -> Black?
    Visited, //visited cells -> Red 0.5 alpha
    Chosen,  //chosen path -> Green 0.5 alpha
    Empty,   //empty cells -> Gray
    End,
    Start,
}

#[derive(Debug, Hash, Eq)]
pub struct CellCoordinates {
    pub x: u16,
    pub y: u16,
}

impl PartialEq for CellCoordinates {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
