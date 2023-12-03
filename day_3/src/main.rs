use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // line: numbers[] - index + number
    // line: symbols[] - index
    // for each line get symbols for prevline, current, next
    // for each number in line check if symbols are adjacent
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut symbol_stack: Vec<Vec<u8>> = Vec::new();
    symbol_stack.push(Vec::new());
    for line_result in reader.lines() {
        let line = line_result?;
        let mut symbols = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                symbols.push(i);
            }
        }
        for i in symbols.iter() {
            print!("{} ", i);
        }
        println!("\n");
    }
    Ok(())
}
