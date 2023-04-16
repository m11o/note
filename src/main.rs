mod git;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitStatus};
use chrono::{Local, NaiveDate};

use crate::git::Git;

fn main() {
    let now = Local::now().date_naive();
    let file_name = build_file_name(&now);

    let status = open_or_create_file_with_vim(&file_name);
    if status.success() {
        Git::add(&file_name);
        Git::commit(&file_name);
    } else {
        println!("Failed to vim")
    }
}

fn build_file_name(date: &NaiveDate) -> String {
    let mut file_name = "./notes/".to_string();
    file_name.push_str(&date.to_string());
    file_name.push_str(".md");

    file_name
}

fn open_or_create_file_with_vim(file_name: &str) -> ExitStatus {
    let path = Path::new(file_name);

    if !path.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_name)
            .expect("Failed to create new file.");

        writeln!(file, "").expect("Failed to write to new file.");
    }

    // Vimでファイルを開く
    Command::new("nvim")
        .arg(file_name)
        .status()
        .expect("Failed to start Vim process.")
}
