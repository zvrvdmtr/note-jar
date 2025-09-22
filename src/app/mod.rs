pub mod repositories;
pub mod services;

use crate::app::repositories::file_repo::FileRepository;
use crate::app::services::notes::{NoteManager, NoteService};
pub struct Application {
    pub notes_service: Box<dyn NoteManager>,
}

impl Application {
    pub fn new() -> Self {
        let repo = Box::new(FileRepository::new());
        let notes_service = Box::new(NoteService::new(repo));
        Application {
            notes_service: notes_service,
        }
    }
}
