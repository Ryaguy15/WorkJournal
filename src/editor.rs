use std::{path::Path, process::Command};

pub fn open_file(path: &Path) {
    Command::new("nvim")
        .arg(path.to_str().unwrap_or(""))
        .status()
        .expect("couldn't open file to edit it");
}
