use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let target = 2020;
    let mut nums: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        let v: i32 = line.unwrap().parse().unwrap();
        nums.push(v)
    }

    // O(N^2) for 3sum soln
    for (i, v) in nums.iter().enumerate() {
        let mut expected: HashMap<i32, _> = HashMap::new();

        for vv in nums[i..].iter() {
            if expected.contains_key(&vv) {
                let ret: i32 = (target - vv - v) * vv * v;
                println!("{} {} {}", v, vv, target - vv - v);
                println!("{:?}", ret);
                return;
            }

            expected.insert(target - vv - v, true);
        }
    }
}
