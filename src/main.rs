use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use datetime::convenience::Today;
use datetime::DatePiece;
use datetime::LocalDate;
use datetime::Month;
use journal::Entry;

mod date_to_string;
mod editor;
mod journal;

#[derive(Parser)]
struct Arg {
    #[arg(value_enum)]
    command: Commands,

    #[arg(short, long)]
    date: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum Commands {
    Open,
    Create,
}

fn get_today_as_filename() -> String {
    let today = LocalDate::today();
    let today_as_string = format!(
        "{}-{}-{}.md",
        today.month().months_from_january() + 1,
        today.day(),
        today.year()
    );
    return today_as_string;
}

fn main() {
    let args = Arg::parse();

    let dir_path = PathBuf::new().join("/Users/ryan/Documents/WorkJournal"); // TODO: make me not
                                                                             // hardcoded

    match args.command {
        Commands::Open => {
            let date = args.date.unwrap_or(get_today_as_filename());
            editor::open_file(dir_path.join(date).as_path());
        }
        Commands::Create => {
            let today = LocalDate::today();
            let entry = Entry::new(today);

            match entry {
                Ok(e) => {
                    let file_path = dir_path.join(e.file_name());
                    match e.write_to_file(dir_path.as_path()) {
                        Ok(_) => editor::open_file(file_path.as_path()),
                        Err(e) => panic!("{:?}", e),
                    };
                }
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            }
        }
    }
}
