use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut cache: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    let mut map_cache: HashMap<Vec<Vec<char>>, Vec<Vec<char>>> = HashMap::new();
    let mut map_positions: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
        rows.push(line.chars().collect());
    }
    let mut repeat = 0;
    for i in 0..1000000000 {
        if map_cache.contains_key(&rows) {
            repeat = i;
            break;
        } else {
            map_positions.insert(rows.clone(), i);
            let before = rows.clone();
            north(&mut rows, &mut cache);
            west(&mut rows, &mut cache);
            south(&mut rows, &mut cache);
            east(&mut rows, &mut cache);
            map_cache.insert(before, rows.clone());
        }
    }
    let start = map_positions.get(&rows).unwrap();
    println!("Repeat at {} - {}", start, repeat);
    let rem = (1000000000 - start) % (repeat - start);
    println!("Iterations left: {}", rem);
    for _j in 0..rem {
        if map_cache.contains_key(&rows) {
            rows = map_cache.get(&rows).unwrap().clone();
        }
    }
    let result_one = calc_load(&rows);
    println!("{}", result_one);
    Ok(())
}

fn calc_load(rows: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for (n, row) in rows.iter().rev().enumerate() {
        for char in row {
            if *char == 'O' {
                load += n + 1;
            }
        }
    }
    load
}

fn roll(col: &Vec<char>, cache: &mut HashMap<Vec<char>, Vec<char>>) -> Vec<char> {
    let cached = cache.get(col);
    match cached {
        Some(x) => x.clone(),
        None => {
            let mut free = 0;
            let mut new_col: Vec<char> = col.clone();
            for (x, char) in col.iter().enumerate() {
                match char {
                    '#' => {
                        free = x + 1;
                    }
                    'O' => {
                        if x > free {
                            new_col[free] = 'O';
                            new_col[x] = '.';
                        }
                        free += 1;
                    }
                    _ => {}
                }
            }
            cache.insert(col.clone(), new_col.clone());
            new_col
        }
    }
}

fn north(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for x in 0..rows[0].len() {
        let col: Vec<char> = (0..rows.len()).map(|y| rows[y][x]).collect();
        let rolled = roll(&col, cache);
        for y in 0..rows.len() {
            rows[y][x] = rolled[y];
        }
    }
}
fn west(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for y in 0..rows.len() {
        let rolled = roll(&rows[y], cache);
        rows[y] = rolled;
    }
}
fn south(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for x in 0..rows[0].len() {
        let col: Vec<char> = (0..rows.len()).map(|y| rows[y][x]).rev().collect();
        let mut rolled = roll(&col, cache);
        rolled.reverse();
        for y in 0..rows.len() {
            rows[y][x] = rolled[y];
        }
    }
}
fn east(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for y in 0..rows.len() {
        let mut original = rows[y].clone();
        original.reverse();
        let mut rolled = roll(&original, cache);
        rolled.reverse();
        rows[y] = rolled;
    }
}
