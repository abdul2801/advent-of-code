use std::fs::File;
use std::io::{BufRead, BufReader};
use main;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut pos = 50;
    let mut buf = String::new();
    let mut res = 0;
    while reader.read_line(&mut buf)? != 0 {
        let (first_char, rest) = buf.trim().split_at(1);
        match first_char {
            "L" => pos -= rest.parse::<i32>()?,
            "R" => pos += rest.parse::<i32>()?,
            _ => {}
        }
       
        pos = ((pos % 100) + 100) % 100;
        if pos == 0 {
            res += 1;
        }
        buf.clear();
    }

    println!("{}", res);
    Ok(())
}
