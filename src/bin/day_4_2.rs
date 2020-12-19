use std::collections::HashMap;
use std::io::{self, BufRead};

fn handle_val(key: &str, val: &str) -> bool {
    match key {
        "byr" => {
            let n: i32 = str::parse(val).unwrap();
            if n < 1920 || n > 2002 {
                return false;
            }
        }
        "iyr" => {
            let n: i32 = str::parse(val).unwrap();
            if n < 2010 || n > 2020 {
                return false;
            }
        }
        "eyr" => {
            let n: i32 = str::parse(val).unwrap();
            if n < 2020 || n > 2030 {
                return false;
            }
        }
        "hgt" => {
            let end = &val[val.len() - 2..];
            match end {
                "cm" => {
                    let n: i32 = str::parse(&val[..val.len() - 2]).unwrap();
                    if n < 150 || n > 193 {
                        return false;
                    }
                }
                "in" => {
                    let n: i32 = str::parse(&val[..val.len() - 2]).unwrap();
                    if n < 59 || n > 76 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        "hcl" => {
            let end = &val[0..1];
            match end {
                "#" => {}
                _ => return false,
            }
            for c in val[1..].chars() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | 'a' | 'b'
                    | 'c' | 'd' | 'e' | 'f' => {}
                    _ => return false,
                }
            }
        }
        "ecl" => match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return false,
        },
        "pid" => {
            if val.len() != 9 {
                return false;
            }
            for c in val.chars() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
                    _ => return false,
                }
            }
        }

        _ => {
            println!("unhandled field {}", key);
            return false;
        }
    }
    return true;
}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    lines.push("".to_string());
    for line in stdin.lock().lines() {
        let v: String = line.unwrap();

        // if the line is not empty, we append it to the previous line
        if v != "" {
            let mut prev = lines.pop().unwrap();
            prev = prev + " " + &v;
            lines.push(prev);
        } else {
            lines.push(v);
        }
    }
    let required: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid = 0;
    'outer: for passport in lines.iter() {
        let mut fields: HashMap<&str, &str> = HashMap::new();

        let split: Vec<&str> = passport.split(' ').collect();
        for field in split.iter() {
            if field.to_string() == "" {
                continue;
            }
            let params: Vec<&str> = field.split(':').collect();
            fields.insert(params[0], params[1]);
        }

        let mut cond_met = true;
        for field in required.iter() {
            if !fields.contains_key(field) {
                continue 'outer;
            }
            let val = fields.get(field).unwrap();
            if !handle_val(field, val) {
                cond_met = false;
            }
        }
        if cond_met {
            valid += 1;
        }
    }
    println!("{}", valid)
}
