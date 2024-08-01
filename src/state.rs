use crate::field::Field;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct SharedState(pub Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> SharedState {
        SharedState(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> MutexGuard<'_, State> {
        return self.0.lock().unwrap();
    }
}

pub struct State {
    pub field: Field,
    pub speed: f64,
}
