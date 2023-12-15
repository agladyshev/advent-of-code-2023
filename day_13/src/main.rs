use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut rows: Vec<String> = Vec::new();
    let mut columns: Vec<String> = Vec::new();
    let mut column_chars: Vec<Vec<char>> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Bad line");
        if n == 0 {
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
                let row_caches: HashMap<usize, u64> = populate_hashmap(&rows);
                let column_caches: HashMap<usize, u64> = populate_hashmap(&columns);
                let mut row_above = 0;
                let mut columns_left = 0;
                println!("{:?}", row_caches);
                println!("{:?}", column_caches);
                //empty line
                //process mirrors
            }
        }
    }
    Ok(())
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
