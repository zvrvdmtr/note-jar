use std::collections::HashSet;

use crate::{app::Application, cli::commands::Command};
use clap::Parser;

use super::commands::BaseCommand;

// TODO: add return as -> Result<(), Error>
pub fn run(app: &Application) {
    let cli = BaseCommand::parse();

    match cli.command {
        Command::Add { note, tags, ttl } => {
            let mut validated_tags = tags.unwrap_or_default();
            validated_tags.dedup();
            app.notes_service.add_note(&note, &validated_tags, ttl);
        }
        Command::List {
            included_tags,
            excluded_tags,
        } => {
            let included = sanitize_tags(included_tags);
            let excluded = sanitize_tags(excluded_tags);

            let intersection: Vec<String> = included.intersection(&excluded).cloned().collect();
            if !intersection.is_empty() {
                println!("{}", intersection.join(", "));
                return;
            }

            let included_list: Vec<_> = included.iter().cloned().collect();
            let excluded_list: Vec<_> = excluded.iter().cloned().collect();

            app.notes_service.notes(&included_list, &excluded_list);
        }
        Command::Delete { id } => app.notes_service.delete_note(&id),
        Command::Get { id } => app.notes_service.get_note(&id),
    }
}

fn sanitize_tags(raw_tags: Option<Vec<String>>) -> HashSet<String> {
    raw_tags.unwrap_or_default().into_iter().collect()
}
