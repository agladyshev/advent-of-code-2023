use std::cmp::min;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut rows: Vec<String> = Vec::new();
    let mut columns: Vec<String> = Vec::new();
    let mut column_chars: Vec<Vec<char>> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Bad line");
        //println!("Line {} {}", n, &line);
        if column_chars.is_empty() {
            let columns_count = line.len();
            for _i in 0..columns_count {
                column_chars.push(Vec::new());
            }
        }
        let first_char = line.chars().next();
        match first_char {
            Some('.') | Some('#') => {
                for (i, char) in line.chars().enumerate() {
                    column_chars[i].push(char);
                }
                rows.push(line);
            }
            None | _ => {
                for chars in &column_chars {
                    let str: String = chars.into_iter().collect();
                    columns.push(str);
                }
                column_chars.clear();
                let row_hashes: HashMap<usize, u64> = populate_hashmap(&rows);
                let column_hashes: HashMap<usize, u64> = populate_hashmap(&columns);
                let rows_above = get_reflected_count(&rows, &row_hashes);
                let columns_left = get_reflected_count(&columns, &column_hashes);
                //println!("{}", rows_above);
                //println!("{}", columns_left);
                result += rows_above * 100 + columns_left;
                columns.clear();
                rows.clear();
            }
        }
    }
    println!("{}", result);
    Ok(())
}

fn get_reflected_count(lines: &Vec<String>, hashes: &HashMap<usize, u64>) -> usize {
    let mut reflections = 0;
    let len = lines.len();
    for i in 1..len {
        //println!("Mirror position: {}", i);
        let depth = min(i, len - i);
        //println!("Depth: {} {} {}", depth, i, len - i);
        let (_before, _after) = lines.split_at(i);
        let mut is_mirror = true;
        for j in 0..depth {
            //println!(
            //    "{} {}",
            //    hashes.get(&(i - j - 1)).expect("out of bounds before"),
            //    hashes.get(&(i + j)).expect("out of bound after")
            //);
            if hashes.get(&(i - j - 1)) != hashes.get(&(i + j)) {
                is_mirror = false;
            }
            //if row_hashes.get[i - j] == row_hashes.get[i + j];
        }
        if is_mirror {
            reflections += i;
        }
        //println!("{}", is_mirror);
    }
    reflections
}

fn populate_hashmap(strs: &Vec<String>) -> HashMap<usize, u64> {
    let mut hashmap: HashMap<usize, u64> = HashMap::new();
    let mut i = 0;
    for str in strs {
        hashmap.insert(i, to_hash(str));
        i += 1;
    }
    hashmap
}

fn to_hash(str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish()
}
