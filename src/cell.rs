use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Tile(pub Arc<Mutex<Cell>>);

impl Eq for Tile {}

//TODO:
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        let a = self.lock().unwrap().coordinates.x;
        let b = other.lock().unwrap().coordinates.x;
        let c = self.lock().unwrap().coordinates.y;
        let d = other.lock().unwrap().coordinates.y;
        a == b && c == d
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
        self.lock().unwrap().state.hash(state);
        self.lock().unwrap().coordinates.hash(state);
    }
}

#[derive(PartialEq, Debug, Hash, Eq)]
pub struct Cell {
    pub state: CellState,
    pub coordinates: CellCoordinates,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell {
        Cell {
            state: CellState::Empty,
            coordinates: CellCoordinates { x, y },
        }
    }
}

#[derive(PartialEq, Debug, Eq, Hash)]
pub enum CellState {
    Blocked, //obstacles -> Black?
    Visited, //visited cells -> Red 0.5 alpha
    Chosen,  //chosen path -> Green 0.5 alpha
    Empty,   //empty cells -> Gray
    TEST,
}

#[derive(PartialEq, Debug, Hash, Eq)]
pub struct CellCoordinates {
    pub x: u16,
    pub y: u16,
}
