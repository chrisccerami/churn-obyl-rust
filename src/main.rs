#[cfg(not(test))]
use std::process::{Command, Output};

fn main() {
    println!("{:?}", git_log().stdout);
}

fn git_log() -> Output {
    Command::new("cd")
        .arg("~/dev/vmm");
    Command::new("git")
        .arg("log")
        .arg("--no-merges")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
}
