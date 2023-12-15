use std::cmp::min;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut result_one = 0;
    let mut result_two = 0;
    let mut rows: Vec<String> = Vec::new();
    let mut columns: Vec<String> = Vec::new();
    let mut column_chars: Vec<Vec<char>> = Vec::new();
    let mut cached_hashes: HashMap<String, u64> = HashMap::new();
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
                //let row_hashes: HashMap<usize, u64> = populate_hashmap(&rows);
                //let column_hashes: HashMap<usize, u64> = populate_hashmap(&columns);
                result_one += get_reflections_value(&rows, &columns, &mut cached_hashes);
                //let row_pairs = find_pairs_with_single_diff(&rows);
                //let column_pairs = find_pairs_with_single_diff(&columns);
                //println!("Rows: {:?}", row_pairs);
                //println!("Columns: {:?}", column_pairs);
                //for pair in row_pairs {
                //    let result = 0;
                //   if result > 0 {
                //      result_two += result;
                //     break;
                //}
                //}
                columns.clear();
                rows.clear();
            }
        }
    }
    println!("{}", result_one);
    //println!("{}", result_two);
    Ok(())
}

fn get_reflections_value(
    rows: &Vec<String>,
    columns: &Vec<String>,
    cached_hashes: &mut HashMap<String, u64>,
) -> usize {
    let rows_above = get_reflected_count(rows, cached_hashes);
    let columns_left = get_reflected_count(columns, cached_hashes);
    rows_above * 100 + columns_left
}
fn get_reflected_count(lines: &Vec<String>, hashes: &mut HashMap<String, u64>) -> usize {
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
            //)
            let left = &lines[i - j - 1];
            let right = &lines[i + j];
            if hashes.get(left).is_none() {
                let hash = to_hash(&left);
                hashes.insert(left.clone(), hash);
                //left_value = Some(&hash);
            }
            if hashes.get(right).is_none() {
                let hash = to_hash(&right);
                hashes.insert(right.clone(), hash);
                //right_value = Some(&hash);
            }
            if hashes.get(left).unwrap() != hashes.get(right).unwrap() {
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

fn is_one_char_diff(s1: &String, s2: &String) -> bool {
    let mut diff_count = 0;
    for (char1, char2) in s1.chars().zip(s2.chars()) {
        if char1 != char2 {
            diff_count += 1;
            if diff_count > 1 {
                return false;
            }
        }
    }
    diff_count == 1
}

fn find_pairs_with_single_diff(strings: &Vec<String>) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            if is_one_char_diff(&strings[i], &strings[j]) {
                pairs.push((i, j));
                pairs.push((j, i));
            }
        }
    }
    pairs
}
