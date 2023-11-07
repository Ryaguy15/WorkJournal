use anyhow::bail;
use anyhow::Result;
use datetime::DatePiece;
use datetime::LocalDate;
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::fs::create_dir;
use std::fs::File;
use std::fs::Permissions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::date_to_string;

#[derive(Clone, Debug)]
pub struct Todo {
    completed: bool,
    description: String,
}

pub struct Entry {
    pub date: String,
    pub todos: Vec<Todo>,
}

#[derive(Debug)]
pub struct EntryParseError {
    reason: String,
}

impl Error for EntryParseError {}
impl Display for EntryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
impl Entry {
    pub fn new(date: LocalDate) -> anyhow::Result<Entry> {
        let yesterday = LocalDate::ymd(date.year(), date.month(), date.day() - 1).ok();

        let mut entry = Entry {
            date: date_to_string::to_filename_string(&date),
            todos: Vec::new(),
        };

        if let Some(y) = yesterday {
            // load yesterdays file and read the uncompleted todos
            let user_home_documents_dir = Path::new("/Users/ryan/Documents");
            let entry_location = user_home_documents_dir.join("WorkJournal");
            if !entry_location.exists() {
                create_dir(entry_location.clone())?;
            } else {
                let yesterday_file_name =
                    entry_location.join(date_to_string::to_filename_string(&y) + ".md"); // TODO:
                                                                                         // Organize getting the date from LocalDate and the filename in a better way
                if yesterday_file_name.exists() {
                    let yesterday_entry_data =
                        EntryFileReader::read(yesterday_file_name.to_str().unwrap().to_string())?;
                    let yeseterday_entry = Self::parse(yesterday_entry_data)?;
                    println!("{:?}", yeseterday_entry.todos);
                    entry.todos = yeseterday_entry.todos.clone();
                }
            }
        }

        return Ok(entry);
    }

    fn parse(data: String) -> anyhow::Result<Entry> {
        let sections: Vec<&str> = data.split("#").collect();
        let todo_section = match sections.get(1) {
            Some(s) => s,
            None => {
                return Err(anyhow::Error::new(EntryParseError {
                    reason: String::from("Failed to find todo section"),
                }));
            }
        };
        println!("{:?}", todo_section);
        let todos: Vec<Todo> = todo_section
            .split("\n")
            .skip(1)
            .map(|line| Todo::parse(line.to_string()))
            .map_while(|t| t.ok())
            .collect();

        return Ok(Entry {
            date: String::new(),
            todos,
        });
    }

    pub fn file_name(&self) -> String {
        return self.date.clone() + ".md";
    }

    pub fn write_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let todo_section = self
            .todos
            .clone()
            .iter()
            .map(|t| t.to_markdown())
            .reduce(|acc, x| acc + "\n" + x.as_str())
            .unwrap_or(String::new());
        let mut file_content = String::from("#TODO\n");
        file_content += todo_section.as_str();

        file_content += "\n#Notes\n";

        let mut file = File::create(path.join(self.file_name()))?;
        file.write_all(file_content.as_bytes())?;
        return Ok(());
    }
}

impl Todo {
    pub fn parse(data: String) -> anyhow::Result<Todo> {
        let is_completed = Regex::new(r"^- \[x\]")?;
        let is_not_completed = Regex::new(r"^- \[\]")?;
        if let Some(_) = is_completed.captures(&data) {
            let description = data.split("- [x]").nth(1).unwrap_or("").to_string();
            return Ok(Todo {
                completed: true,
                description,
            });
        } else {
            if let Some(_) = is_not_completed.captures(&data) {
                let description = data.split("- []").nth(1).unwrap_or("").to_string();
                return Ok(Todo {
                    completed: false,
                    description,
                });
            } else {
                println!("{:?}", data);
                return Err(anyhow::Error::new(EntryParseError {
                    reason: String::from("Failed to parse todo line"),
                }));
            }
        }
    }

    pub fn to_markdown(&self) -> String {
        if self.completed {
            return String::from("- [x]") + self.description.as_str();
        } else {
            return String::from("- []") + self.description.as_str();
        }
    }
}

pub struct EntryFileReader;

impl EntryFileReader {
    pub fn read(file_path: String) -> Result<String> {
        let mut file = File::open(file_path)?;
        let mut file_contents = String::new();

        let _ = file.read_to_string(&mut file_contents);

        return Ok(file_contents);
    }
}
