use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let v: String = line.unwrap();
        lines.push(v)
    }
    let end = lines.len();
    let width = lines[0].len();

    let rules: Vec<Vec<usize>> = vec![vec![1, 1], vec![1, 3], vec![1, 5], vec![1, 7], vec![2, 1]];
    let mut trees: Vec<u32> = Vec::new();
    for rule in rules {
        let mut tree = 0;
        let mut row = 0;
        let mut leftpos = 0;
        while row < end {
            let line = &lines[row];
            if line.as_bytes()[leftpos % width] == b'#' {
                tree += 1;
            }
            leftpos += rule[1];
            row += rule[0];
        }
        trees.push(tree);
    }
    println!("counts: {:?}", trees);

    let mut res: u64 = 1;
    for t in trees {
        res = res * u64::from(t);
    }
    println!("got: {}", res)
}
