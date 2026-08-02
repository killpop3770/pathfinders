use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use crate::domain::field::Field;

#[derive(Clone)]
pub struct SharedState {
    inner: Arc<Mutex<State>>,
}

impl SharedState {
    pub fn new(state: State) -> Self {
        Self {
            inner: Arc::new(Mutex::new(state)),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, State> {
        self.inner.lock().unwrap()
    }

    pub fn speed(&self) -> f64 {
        self.lock().speed
    }

    pub fn should_stop(&self) -> bool {
        self.lock().stop_flag
    }

    pub fn set_stop_flag(&self, value: bool) {
        self.lock().stop_flag = value;
    }

    pub fn wait(&self, base_ms: f64, speed_multiplier: f64) {
        if speed_multiplier <= 0.0 {
            return;
        }

        let delay_ms = (base_ms / speed_multiplier).max(0.0);

        if delay_ms > 0.0 {
            std::thread::sleep(Duration::from_millis(delay_ms as u64));
        }
    }
}

pub struct State {
    field: Field,
    pub speed: f64,
    pub stop_flag: bool,
}

impl State {
    pub fn new(field: Field, speed: f64) -> Self {
        Self {
            field,
            speed,
            stop_flag: false,
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn field_mut(&mut self) -> &mut Field {
        &mut self.field
    }
}
