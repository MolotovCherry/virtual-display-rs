use std::sync::Arc;

use driver_ipc::Monitor;

#[derive(Debug)]
pub enum Action {
    Add(Arc<Monitor>),
    Remove(Arc<Monitor>),
    RemoveAll,
}

impl Action {
    pub fn play(&self) {
        match self {
            Action::Add(monitor) => todo!(),
            Action::Remove(id) => todo!(),
            Action::RemoveAll => todo!(),
        }
    }
}

pub trait PlayActions {
    fn play_actions(&mut self);
}

impl PlayActions for Vec<Action> {
    fn play_actions(&mut self) {
        for action in self.drain(..) {}
    }
}


