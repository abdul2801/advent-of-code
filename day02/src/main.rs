use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut pos: i32 = 50;
    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let (dir, rest) = line.split_at(1);
        let amount: i32 = rest.parse()?;

        let old_pos = pos;

        match dir {
            "L" => pos -= amount,
            "R" => pos += amount,
            _ => {}
        }

        if pos > 99 {
            result += pos / 100;
        }

        if pos < 1 {
            result += ((old_pos - 1) / 100) - ((pos - 1) / 100);
        }

        pos = pos.rem_euclid(100);

       
    }

    println!("{result}");
    Ok(())
}
