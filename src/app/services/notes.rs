use chrono::NaiveDate;

pub trait NoteManager {
    fn get_note(&self, id: &str);
    fn notes(&self, included_tags: &[String], excluded_tags: &[String]);
    fn add_note(&self, text: &str, tags: &[String], expiry_date: Option<NaiveDate>);
    fn delete_note(&self, id: &str);
}

pub trait FileManager {
    fn read_file(&self, id: &str);
    fn list_files(&self, included_tags: &[String], excluded_tags: &[String]);
    fn save_file(&self, content: &str, tags: &[String], expiry_date: Option<NaiveDate>);
    fn delete_file(&self, id: &str);
}

pub struct Note {
    text: String,
    tags: Vec<String>,
    ttl: NaiveDate,
}

pub struct NoteService {
    file_manager: Box<dyn FileManager>,
}

impl NoteService {
    pub fn new(file_manager: Box<dyn FileManager>) -> Self {
        return NoteService {
            file_manager: file_manager,
        };
    }
}

impl NoteManager for NoteService {
    fn get_note(&self, id: &str) {
        self.file_manager.read_file(id);
    }
    fn notes(&self, included_tags: &[String], excluded_tags: &[String]) {
        self.file_manager.list_files(included_tags, excluded_tags);
    }
    fn add_note(&self, text: &str, tags: &[String], expiry_date: Option<NaiveDate>) {
        self.file_manager.save_file(text, tags, expiry_date);
    }
    fn delete_note(&self, id: &str) {
        self.file_manager.delete_file(id);
    }
}
