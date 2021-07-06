use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn file_read_all_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect("No such file found.");
    return BufReader::new(file)
        .lines()
        .map(|l| l.expect("Line could not be parsed."))
        .collect();
}