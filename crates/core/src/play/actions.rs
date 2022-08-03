use std::fmt::Display;

use crate::note::CompiledNote;

#[derive(Debug, Clone)]
pub enum ActionState {
    Miss,
    Boo,
    Hit,
}

#[derive(Debug, Clone)]
pub struct NoteAction {
    pub note: CompiledNote,
    pub timestamp: i128,
    pub state: ActionState,
}

impl Display for NoteAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.note)
    }
}
