use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut result_one = 0;
    let mut cache: HashMap<String, String> = HashMap::new();
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
        rows.push(line.chars().collect());
    }
    for row in &rows {
        println!("{}", row.iter().collect::<String>());
    }
    let row_len = rows[0].len();
    let col_len = rows.len();
    //for i in 0..1000000000 {
    north(&mut rows, &mut cache, col_len);
    //    west(&mut rows, &mut cache);
    //    south(&mut rows, &mut cache, col_len);
    //    east(&mut rows, &mut cache);
    //}
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

fn north(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<String, String>, col_len: usize) {
    for x in 0..rows[0].len() {
        let str_before: String = (0..col_len).map(|i| rows[i][x]).collect();
        if cache.contains_key(&str_before) {
            let str_after = cache.get(&str_before).expect("No value");
            for y in 0..rows.len() {
                rows[y][x] = str_after.chars().nth(y).unwrap();
            }
        } else {
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
            let str_after: String = (0..col_len).map(|i| rows[i][x]).collect();
            cache.insert(str_before, str_after);
        }
    }
}
fn west(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<String, String>) {
    for row in rows {
        let str_before: String = row.iter().collect();
        if cache.contains_key(&str_before) {}
    }
}
fn south(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<String, String>, col_len: usize) {}
fn east(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<String, String>) {}
