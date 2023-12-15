use std::cmp::min;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result_one = 0;
    let mut result_two = 0;
    let mut rows: Vec<String> = Vec::new();
    let mut columns: Vec<String> = Vec::new();
    let mut column_chars: Vec<Vec<char>> = Vec::new();
    let mut cached_hashes: HashMap<String, u64> = HashMap::new();
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
        if column_chars.is_empty() {
            let columns_count = line.len();
            for _i in 0..columns_count {
                column_chars.push(Vec::new());
            }
        }
        let first_char = line.chars().next();
        match first_char {
            Some(_) => {
                for (i, char) in line.chars().enumerate() {
                    column_chars[i].push(char);
                }
                rows.push(line);
            }
            None => {
                for chars in &column_chars {
                    let str: String = chars.into_iter().collect();
                    columns.push(str);
                }
                let old_row_reflections = get_reflections(&rows, &mut cached_hashes);
                let old_column_reflections = get_reflections(&columns, &mut cached_hashes);
                let old_result = old_row_reflections.iter().map(|&x| x * 100).sum::<usize>()
                    + old_column_reflections.iter().sum::<usize>();
                result_one += old_result;
                let mut new_result = 0;
                let row_pairs = find_pairs_with_single_diff(&rows);
                let column_pairs = find_pairs_with_single_diff(&columns);
                assert!(row_pairs.len() > 0 || column_pairs.len() > 0);
                for pair in row_pairs {
                    let mut updated_rows = rows.clone();
                    let mut updated_columns = columns.clone();
                    updated_rows[pair.0] = updated_rows[pair.1].clone();
                    for i in 0..updated_columns.len() {
                        let mut chars = column_chars[i].clone();
                        chars[pair.0] = chars[pair.1];
                        updated_columns[i] = chars.into_iter().collect();
                    }
                    new_result = find_new_reflections(
                        &updated_rows,
                        &updated_columns,
                        &old_row_reflections,
                        &old_column_reflections,
                        &mut cached_hashes,
                    );
                    if new_result > 0 {
                        result_two += new_result;
                        break;
                    }
                }
                if new_result == 0 {
                    for pair in column_pairs {
                        let mut updated_rows = rows.clone();
                        let mut updated_columns = columns.clone();
                        updated_columns[pair.0] = updated_columns[pair.1].clone();
                        for i in 0..updated_rows.len() {
                            let mut chars: Vec<char> = updated_rows[i].chars().collect();
                            chars[pair.0] = chars[pair.1];
                            updated_rows[i] = chars.into_iter().collect();
                        }
                        new_result = find_new_reflections(
                            &updated_rows,
                            &updated_columns,
                            &old_row_reflections,
                            &old_column_reflections,
                            &mut cached_hashes,
                        );
                        if new_result > 0 {
                            result_two += new_result;
                            break;
                        }
                    }
                }
                assert!(new_result > 0);
                column_chars.clear();
                columns.clear();
                rows.clear();
            }
        }
    }
    println!("{}", result_one);
    println!("{}", result_two);
    Ok(())
}

fn find_new_reflections(
    updated_rows: &Vec<String>,
    updated_columns: &Vec<String>,
    old_row_reflections: &Vec<usize>,
    old_column_reflections: &Vec<usize>,
    cached_hashes: &mut HashMap<String, u64>,
) -> usize {
    let mut new_row_reflections = get_reflections(&updated_rows, cached_hashes);
    let mut new_column_reflections = get_reflections(&updated_columns, cached_hashes);
    new_row_reflections = new_row_reflections
        .into_iter()
        .filter(|x| !old_row_reflections.contains(x))
        .collect::<Vec<usize>>();
    new_column_reflections = new_column_reflections
        .into_iter()
        .filter(|x| !old_column_reflections.contains(x))
        .collect::<Vec<usize>>();
    if new_row_reflections.len() != 0 || new_column_reflections.len() != 0 {
        return new_row_reflections.iter().map(|&x| x * 100).sum::<usize>()
            + new_column_reflections.iter().sum::<usize>();
    }
    0
}

fn get_reflections(lines: &Vec<String>, hashes: &mut HashMap<String, u64>) -> Vec<usize> {
    let mut reflections: Vec<usize> = Vec::new();
    let len = lines.len();
    for i in 1..len {
        let depth = min(i, len - i);
        let (_before, _after) = lines.split_at(i);
        let mut is_mirror = true;
        for j in 0..depth {
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
        }
        if is_mirror {
            reflections.push(i);
        }
    }
    reflections
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
