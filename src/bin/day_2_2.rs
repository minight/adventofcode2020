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

        let mut matches: u32 = 0;
        if v[2].as_bytes()[min - 1] == letter.as_bytes()[0] {
            matches += 1;
        }
        if v[2].as_bytes()[max - 1] == letter.as_bytes()[0] {
            matches += 1;
        }
        if matches == 1 {
            valid += 1
        }
    }
    println!("valid: {}", valid)
}
