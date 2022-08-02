use std::{fmt::Display, time::Duration};

use crate::note::CompiledNote;

#[derive(Debug)]
pub enum ActionState {
    Miss,
    Boo,
    Hit,
}

#[derive(Debug)]
pub struct NoteAction {
    pub note: CompiledNote,
    pub timestamp: Duration,
    pub state: ActionState,
}

impl Display for NoteAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.note)
    }
}
