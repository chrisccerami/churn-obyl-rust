use std::fs::File;
use std::io::{BufRead,BufReader,Write};
use std::collections::HashMap;
extern crate rustc_serialize;
use rustc_serialize::json;
extern crate regex;
use regex::Regex;

fn main() {
    let mut counted_files = most_often_changed_files();
    counted_files.truncate(100);
    let mut json_file = File::create("git-log.json").unwrap();
    if let Ok(json) = json::encode(&counted_files) {
        match json_file.write(json.as_bytes()) {
            Ok(_) => println!("Most commonly changed files written to git-log.json"),
            Err(_) => println!("Could not write to git-log.json")
        }
    }
}

#[derive(RustcEncodable)]
struct FileCount {
    filename: String,
    count: u32
}

fn most_often_changed_files() -> Vec<FileCount> {
    let mut files: HashMap<String, u32> = HashMap::new();
    for file in &changed_files() {
        let count = files.entry(file.clone()).or_insert(0);
        *count += 1;
    }
    let counts: Vec<(String, u32)> = files.into_iter().collect();
    let mut file_counts: Vec<FileCount> = counts.into_iter().map(|count| FileCount{filename: count.0, count: count.1} ).collect();
    file_counts.sort_by(|a, b| a.count.cmp(&b.count).reverse() );
    file_counts
}

fn changed_files() -> Vec<String> {
    let lines = BufReader::new(File::open("log.txt").unwrap()).lines();
    let re = Regex::new(r"^(\w+/)*\w+\.\w+$").unwrap();
    let commit_lines = lines.map(|line| line.unwrap());
    commit_lines.filter(|line| re.is_match(line)).collect()
}
