use std::io::{self, BufRead};
use std::collections::HashMap;

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
        } else  {
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
                continue
            }
            let params: Vec<&str> = field.split(':').collect();
            fields.insert(params[0], params[1]);
        }
        for field in required.iter() {
            if !fields.contains_key(field) {
                continue 'outer;
            }
        }
        valid +=1 ;
    }
    println!("{}", valid)
}
