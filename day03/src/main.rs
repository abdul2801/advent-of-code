use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut res = 0;
    while reader.read_line(&mut buf)? != 0 {
        let line = buf.trim();
        
        let cur = find_max(line);
        
        res += cur;
        
        buf.clear();
    }

    println!("{}", res);
    Ok(())
}

fn find_max(input: &str) -> i64 {
    let mut best = -1;
    let mut future_best = -1;
    for line in input.chars().rev() {
        let num = line.to_digit(10).unwrap_or(0) as i64;
        
        if future_best != -1 {
            best = best.max(num * 10 + future_best);
        }
        
        future_best = future_best.max(num);
    }
    best
}
