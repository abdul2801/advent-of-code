use std::fs::{self};
use fancy_regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut c = 0;
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    for instruction in input.split(",") {
        if let Some((l, r)) = instruction.split_once('-') {
            let left = l.parse::<i64>()?;
            let right = r.parse::<i64>()?;
            let invalid = find_invalid(left, right, &re);
            println!("Invalid numbers: {}", invalid);
            c +=invalid;
        }
    }
    // println!("Invalid numbers: {}", is_invalid2(12312));
    println!("Total invalid numbers: {}", c);
    
    Ok(())
}


fn find_invalid(left: i64, right: i64, re: &Regex) -> i64 {
    let mut invalid = 0;
    for i in left..=right {
        if is_invalid2(i, re) {
            invalid += i;
        }
    }
    invalid
}

fn is_invalid(num: i64) -> bool {
    let s = num.to_string();
    let n = s.len();
    if n >= 2 && s[0..(n/2)] == s[(n/2)..] {
        true
    } else {
        false
    }
}
fn is_invalid2(num: i64, re: &Regex) -> bool {
    let s = num.to_string();
    re.is_match(&s).unwrap()
}
