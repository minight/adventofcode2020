use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let target = 2020;
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let v: String = line.unwrap();
        lines.push(v)
    }
    let end = lines.len();
    let width = lines[0].len();

    let mut leftpos = 0;
    let mut trees = 0;
    for line in lines.iter() {
        if line.as_bytes()[leftpos % width] == b'#' {
            trees += 1;
        }
        leftpos += 3;
    }
    println!("trees {}", trees)
}
