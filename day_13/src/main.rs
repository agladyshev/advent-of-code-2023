use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut row_caches: HashMap<usize, u64> = HashMap::new();
    let mut column_caches: HashMap<usize, u64> = HashMap::new();
    let mut rows: Vec<String> = Vec::new();
    let mut columns: Vec<Vec<char>> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Bad line");
        if n == 0 {
            let columns_count = line.len();
            for _i in 0..columns_count {
                columns.push(Vec::new());
            }
        }
        let first_char = line.chars().next();
        match first_char {
            Some('.') | Some('#') => {
                for (i, char) in line.chars().enumerate() {
                    columns[i].push(char);
                }
                rows.push(line);
            }
            None | _ => {
                {
                    let mut i = 0;
                    for str in &rows {
                        row_caches.insert(i, to_hash(str));
                        i += 1;
                    }
                }
                {
                    let mut i = 0;
                    for column in &columns {
                        let str: String = column.into_iter().collect();
                        column_caches.insert(i, to_hash(&str));
                        i += 1;
                    }
                }
                println!("{:?}", row_caches);
                println!("{:?}", column_caches);
                //empty line
                //process mirrors
                //
            }
        }
    }
    Ok(())
}

fn to_hash(str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish()
}
