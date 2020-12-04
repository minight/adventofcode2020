use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut expected = HashMap::new();
    let target: u32 = 2020;
    for line in stdin.lock().lines() {
        let v: u32 = line.unwrap().parse().unwrap();
        if expected.contains_key(&v) {
            let ret: u32 = (target - v) * v;
            println!("{:?}", ret);
            return;
        }

        expected.insert(target - v, true);
    }
}
