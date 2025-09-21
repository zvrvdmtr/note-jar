use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about=None)]
pub struct BaseCommand {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Add new note")]
    Add {
        #[arg(help = "Text of your note")]
        note: String,
        #[arg(
            long,
            value_delimiter = ',',
            help = "List of comma separated tags. Example: --tags=first,second",
            value_name = "[TAGS]"
        )]
        tags: Option<Vec<String>>,
        #[arg(
            long,
            help = "Date until note is actual. Use naive format. Example: --ttl=1970-01-01",
            value_name = "DATE"
        )]
        ttl: Option<NaiveDate>,
    },
    #[command(about = "Get note by it ids")]
    Get {
        #[arg(long, help = "Will find note with this particular id")]
        id: String,
    },
    #[command(about = "Filter list of notes")]
    List {
        #[arg(
            long,
            help = "Include only notes with these tags. Example: --included-tags=first,second",
            value_name = "[TAGS]",
            value_delimiter = ','
        )]
        included_tags: Option<Vec<String>>,
        #[arg(
            long,
            help = "Exclude only notes with these tags. Example: --excluded-tags=first,second",
            value_name = "[TAGS]",
            value_delimiter = ','
        )]
        excluded_tags: Option<Vec<String>>,
    },
    #[command(about = "Delete note by it's id")]
    Delete {
        #[arg(long, help = "Will delete note with this particular id")]
        id: String,
    },
}
