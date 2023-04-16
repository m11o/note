/*
1. file存在確認
2. fileがなければ、作成
3. fileをvimで開く
4. 閉じた後、commitする
5. どっかでpushする
 */
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

use chrono::{Local, NaiveDate};

fn main() {
    let now = Local::now().date_naive();
    let file_name = buuld_file_name(&now);

    open_or_create_file_with_vim(&file_name);
}

fn buuld_file_name<'a>(date: &'a NaiveDate) -> String {
    let mut file_name = "./notes/".to_string();
    file_name.push_str(&date.to_string());
    file_name.push_str(".md");

    file_name
}

fn open_or_create_file_with_vim(file_name: &str) {
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
    let status = Command::new("nvim")
        .arg(file_name)
        .status()
        .expect("Failed to start Vim process.");

    // Vimの実行結果を待機する
    if status.success() {
        git_add(file_name).expect("Failed to git add for file");
        git_commit("update").expect("Failed to commit");
    } else {
        println!("Vim closed with an error.");
    }
}

fn git_add(file_name: &str) -> Result<Output, std::io::Error> {
  let output = Command::new("git")
      .arg("add")
      .arg(file_name)
      .output()?;
    Ok(output)
}

fn git_commit(commit_message: &str) -> Result<Output, std::io::Error> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()?;
    Ok(output)
}

