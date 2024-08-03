use crate::state::SharedState;

pub mod proto;

pub trait Algorithm {
    fn search(&self, state: SharedState);
}
