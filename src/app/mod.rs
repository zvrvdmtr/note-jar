pub mod services;

use crate::app::services::notes::{NoteService, NoteManager};
pub struct Application {
    pub note_service: Box<dyn NoteManager>
}

impl Application {
    pub fn new() -> Self {
        Application {
            note_service: Box::new(NoteService::new())
        }
    }
}