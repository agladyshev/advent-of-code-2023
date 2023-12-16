use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result_one = 0;
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
        rows.push(line.chars().collect());
    }
    for row in &rows {
        println!("{}", row.iter().collect::<String>());
    }
    for x in 0..rows[0].len() {
        let mut free = 0;
        for y in 0..rows.len() {
            let char = rows[y][x];
            match char {
                '#' => {
                    free = y + 1;
                }
                'O' => {
                    if y > free {
                        rows[free][x] = 'O';

                        rows[y][x] = '.';
                    }
                    free += 1;
                }
                _ => {}
            }
        }
    }
    println!();
    for row in &rows {
        println!("{}", row.iter().collect::<String>());
    }
    for (n, row) in rows.iter().rev().enumerate() {
        for char in row {
            if *char == 'O' {
                result_one += n + 1;
            }
        }
    }
    println!("{}", result_one);
    Ok(())
}
