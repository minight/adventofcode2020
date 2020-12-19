use std::collections::HashMap;
use std::io::{self, BufRead};

fn calculatePos(start: i32, operations: &[Operation]) -> i32 {
    let mut min: i32 = 0;
    let mut max: i32 = start;
    for op in operations.iter() {
        match op {
            Operation::left => {
                max = (max + min)/2 ;
            }
            Operation::right => {
                min = (max + min)/ 2 + 1;
            }
        }
    }
    if min != max {
        println!(
            "wtf, we dont have a midpoint: min:{} max:{} ops:{:?} start:{}",
            min, max, operations, start
        );
        return -1;
    }
    return min;
}

#[derive(Debug)]
enum Operation {
    left,
    right,
}

fn calculate(instr: &str) -> (i32, i32) {
    let mut ops: Vec<Operation> = Vec::new();
    for v in instr[0..7].chars() {
        match v {
            'F' => ops.push(Operation::left),
            'B' => ops.push(Operation::right),
            _ => {
                println!("unexpected op {}", v)
            }
        }
    }
    let FB = calculatePos(127, &ops);

    ops.clear();

    for v in instr[7..].chars() {
        match v {
            'L' => ops.push(Operation::left),
            'R' => ops.push(Operation::right),
            _ => {
                println!("unexpected  op {}", v)
            }
        }
    }
    let LR = calculatePos(7, &ops);
    return (FB, LR);

}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    lines.push("".to_string());
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }
    let mut max = 0;
    for line in lines.iter() {
        if line == "" {
            continue;
        }
        let (fb, lr) = calculate(line);
        let res = fb * 8 + lr;
        if res > max {
            max = res
        }
    }
    println!("{}", max)
}
