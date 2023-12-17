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
    //for row in &rows {
    //    println!("{}", row.iter().collect::<String>());
    //}
    let col_len = rows.len();
    let mut repeat = 0;
    for i in 0..1000000000 {
        //if calc_load(&rows) == 64 {
        //    println!("64 at {}", i);
        //}
        if map_cache.contains_key(&rows) {
            repeat = i;
            break;
            rows = map_cache.get(&rows).unwrap().clone();
        } else {
            map_positions.insert(rows.clone(), i);
            let before = rows.clone();
            north(&mut rows, &mut cache, col_len);
            west(&mut rows, &mut cache);
            south(&mut rows, &mut cache, col_len);
            east(&mut rows, &mut cache);
            map_cache.insert(before, rows.clone());
        }
    }
    let start = map_positions.get(&rows).unwrap();
    println!("Repeat at {} - {}", start, repeat);
    //for row in &rows {
    //    println!("{}", row.iter().collect::<String>());
    //}
    let rem = (1000000000 - start) % (repeat - start);
    println!("{}", rem);
    for j in 0..rem {
        //if calc_load(&rows) == 64 {
        //    println!("64 at {}", j);
        //}
        if map_cache.contains_key(&rows) {
            rows = map_cache.get(&rows).unwrap().clone();
        } else {
            println!("!!!!");
            let before = rows.clone();
            north(&mut rows, &mut cache, col_len);
            west(&mut rows, &mut cache);
            south(&mut rows, &mut cache, col_len);
            east(&mut rows, &mut cache);
            map_cache.insert(before, rows.clone());
        }
    }
    let result_one = calc_load(&rows);

    println!("Strings: {:?}", cache.len());
    println!("Maps: {:?}", map_cache.len());
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

fn north(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>, col_len: usize) {
    for x in 0..rows[0].len() {
        //let str_before: Vec<char> = (0..col_len).map(|i| rows[i][x]).collect();
        //if cache.contains_key(&str_before) {
        //    let str_after = cache.get(&str_before).expect("No value");
        //    for y in 0..rows.len() {
        //        rows[y][x] = str_after[y];
        //    }
        //} else {
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
        // let str_after: Vec<char> = (0..col_len).map(|i| rows[i][x]).collect();
        // cache.insert(str_before, str_after);
        // }
    }
}
fn west(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for y in 0..rows.len() {
        //if cache.contains_key(&rows[y]) {
        //    let cached = cache.get(&rows[y]).expect("no value").to_vec();
        //for x in 0..cached.len() {
        //        rows[y][x] = cached[x];
        //    }
        //} else {
        //let vec_before = rows[y].clone();
        let mut free = 0;
        for x in 0..rows[y].len() {
            let char = rows[y][x];
            match char {
                '#' => {
                    free = x + 1;
                }
                'O' => {
                    if x > free {
                        rows[y][free] = 'O';
                        rows[y][x] = '.';
                    }
                    free += 1;
                }
                _ => {}
            }
        }
        //    cache.insert(vec_before, rows[y].clone());
        //}
    }
}
fn south(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>, col_len: usize) {
    // cache reverse
    for x in 0..rows[0].len() {
        //let str_before: Vec<char> = (0..col_len).map(|i| rows[i][x]).rev().collect();
        //if cache.contains_key(&str_before) {
        //    let str_after = cache.get(&str_before).expect("No value");
        //    for y in 0..rows.len() {
        //        rows[y][x] = str_after[y];
        //    }
        //} else {
        let mut free = rows.len() - 1;
        let mut y = rows.len() - 1;
        loop {
            let char = rows[y][x];
            match char {
                '#' => {
                    if y == 0 {
                        free = 0;
                    } else {
                        free = y - 1;
                    }
                }
                'O' => {
                    if y < free {
                        rows[free][x] = 'O';
                        rows[y][x] = '.';
                    }
                    if free > 0 {
                        free -= 1;
                    } else {
                        free = 0;
                    }
                }
                _ => {}
            }
            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
        //  let str_after: Vec<char> = (0..col_len).map(|i| rows[i][x]).rev().collect();
        //  cache.insert(str_before, str_after);
        // }
    }
}
fn east(rows: &mut Vec<Vec<char>>, cache: &mut HashMap<Vec<char>, Vec<char>>) {
    for y in 0..rows.len() {
        //let mut vec_before = rows[y].clone();
        //vec_before.reverse();
        //if cache.contains_key(&vec_before) {
        //    let cached = cache.get(&vec_before).expect("no value").to_vec();
        //    for x in 0..cached.len() {
        //        rows[y][x] = cached[x];
        //    }
        //} else {
        let mut free = rows[y].len() - 1;
        let mut x = rows[y].len() - 1;
        loop {
            let char = rows[y][x];
            match char {
                '#' => {
                    if x == 0 {
                        free = 0;
                    } else {
                        free = x - 1;
                    }
                }
                'O' => {
                    if x < free {
                        rows[y][free] = 'O';
                        rows[y][x] = '.';
                    }
                    if free > 0 {
                        free -= 1;
                    } else {
                        free = 0;
                    }
                }
                _ => {}
            }
            if x == 0 {
                break;
            } else {
                x -= 1;
            }
        }
        //  let mut vec_after = rows[y].clone();
        // vec_after.reverse();
        // cache.insert(vec_before, vec_after);
        // }
    }
}
