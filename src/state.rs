use std::sync::{Arc, Mutex, MutexGuard};

use crate::field::Field;

#[derive(Clone)]
pub struct SharedState(pub Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> SharedState {
        SharedState(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> MutexGuard<'_, State> {
        return self.0.lock().unwrap();
    }

    // pub fn wait(&self) {
    //     let a= 1000.0 / self.get().speed;
    //     println!("A {a}");
    //     thread::sleep(Duration::from_nanos((a) as u64));
    // }
}

pub struct State {
    field: Field,
    speed: f64,
}

impl State {
    pub fn new(field: Field, speed: f64) -> State {
        State { field, speed }
    }

    pub fn field(&mut self) -> &mut Field {
        return &mut self.field;
    }
}
