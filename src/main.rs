use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::EncoderError;
extern crate regex;
use regex::Regex;
extern crate hyper;
use hyper::server::{Server, Request, Response};
use hyper::header::AccessControlAllowOrigin;

fn main() {
    Server::http("0.0.0.0:9876").unwrap().handle(handler).unwrap();
}

fn handler(_: Request, mut response: Response) {
    response.headers_mut().set(AccessControlAllowOrigin::Any);
    response.send(response_json().unwrap().as_bytes()).unwrap();
}

fn response_json() -> Result<String, EncoderError> {
    let mut counted_files = most_often_changed_files();
    counted_files.truncate(100);
    json::encode(&counted_files)
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
