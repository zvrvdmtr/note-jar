use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use crate::app::services::notes::FileManager;

pub struct FileRepository {}

const DATE_TIME_FORMAT: &str = "%Y%m%d%H%M%S";

impl FileRepository {
    pub fn new() -> Self {
        FileRepository {}
    }
}

impl FileManager for FileRepository {
    fn read_file(&self, id: &str) {
        println!("{}", id);
    }
    fn list_files(&self, included_tags: &[String], excluded_tags: &[String]) {
        println!("{:?}", included_tags);
        println!("{:?}", excluded_tags);
    }
    fn delete_file(&self, id: &str) {
        println!("{}", id);
    }
    fn save_file(&self, content: &str, tags: &[String], expiry_date: Option<chrono::NaiveDate>) {
        let body = build_content(&build_front_matter(tags, expiry_date), content);

        let mut file = File::create(build_file_name(content)).expect("ERROR");
        file.write_all(body.as_bytes()).unwrap();
    }
}

fn build_front_matter(tags: &[String], expiry_date: Option<chrono::NaiveDate>) -> String {
    let date = expiry_date.map(|value| value.to_string()).unwrap_or_default();
    format!("---\ntags: [{}]\nexpiry_date: {}\n---\n", tags.join(", "), date)
}

fn build_content(front_matter: &str, content: &str) -> String {
    format!("{}{}", front_matter, content)
}

fn build_file_name(content: &str) -> String {
    let timestamp = chrono::Utc::now().format(DATE_TIME_FORMAT);
    let mut hash = DefaultHasher::new();
    content.hash(&mut hash);
    let result = hash.finish();
    format!("{}-{}.md", timestamp, result)
}
