use chrono::NaiveDate;

pub trait NoteManager {
    fn get_note(&self, id: &str);
    fn notes(&self, included_tags: &[String], excluded_tags: &[String]);
    fn add_note(&self, text: &str, tags: &[String], ttl: Option<NaiveDate>);
    fn delete_note(&self, id: &str);
}

pub trait FileManager {
    fn read_file(&self, id: &str);
    fn list_files(&self, included_tags: &[String], excluded_tags: &[String]);
    fn save_file(&self, content: &str, tags: &[String], ttl: Option<NaiveDate>);
    fn delete_file(&self, id: &str);
}

pub struct Note {
    text: String,
    tags: Vec<String>,
    ttl: NaiveDate,
}

pub struct NoteService {}

impl NoteService {
    pub fn new() -> Self {
        return NoteService {};
    }
}

impl NoteManager for NoteService {
    fn get_note(&self, id: &str) {
        println!("{}", id);
    }
    fn notes(&self, included_tags: &[String], excluded_tags: &[String]) {
        println!("{:?}", included_tags);
        println!("{:?}", excluded_tags);
    }
    fn add_note(&self, text: &str, tags: &[String], ttl: Option<NaiveDate>) {
        println!("{}", text);
        println!("{:?}", tags);
        println!("{:?}", ttl);
    }
    fn delete_note(&self, id: &str) {
        println!("{}", id);
    }
}
