use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut valid: u32 = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.splitn(3, ' ').collect();
        let range: Vec<&str> = v[0].splitn(2, '-').collect();

        let min: usize = range[0].parse().unwrap();
        let max: usize = range[1].parse().unwrap();
        let letter = v[1].strip_suffix(":").unwrap();
        let exists: Vec<&str> = v[2].matches(letter).collect();
        if min <= exists.len() && exists.len() <= max {
            valid += 1;
        }
    }
    println!("valid: {}", valid)
}
