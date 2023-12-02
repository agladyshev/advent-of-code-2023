use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static NUMBERS: &[(&str, u8)] = &[
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

static NUMBERS_REVERSED: &[(&str, u8)] = &[
    ("orez", 0),
    ("eno", 1),
    ("owt", 2),
    ("eerht", 3),
    ("ruof", 4),
    ("evif", 5),
    ("xis", 6),
    ("neves", 7),
    ("thgie", 8),
    ("enin", 9),
];

fn main() -> std::io::Result<()> {
    let map: HashMap<_, _> = NUMBERS.iter().cloned().collect();
    let map_reversed: HashMap<_, _> = NUMBERS_REVERSED.iter().cloned().collect();
    let mut total: u16 = 0;
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let num_1 = find_first_num(&line, &map);
        let num_2 = find_first_num(&line.chars().rev().collect::<String>(), &map_reversed);
        let mut line_value = 0;
        line_value += (num_1.expect("num_1 is empty") * 10) as u16;
        line_value += num_2.expect("num_2 is empty") as u16;
        println!("{} {}", line, line_value);
        total += line_value;
    }
    println!("{}", total);
    Ok(())
}

fn find_first_num(line: &String, map: &HashMap<&str, u8>) -> Option<u8> {
    let mut str = String::new();
    for byte in line.bytes() {
        if is_num(byte) {
            return Some(ascii_to_int(byte).expect("not a valid digit"));
        } else {
            str.push(char::from_u32(byte as u32).expect("not a valid char"));
            if str.len() >= 3 {
                for (key, value) in map {
                    if str.contains(key) {
                        return Some(*value);
                    }
                }
            }
        }
    }
    None
}

fn is_num(byte: u8) -> bool {
    if byte >= b'0' && byte <= b'9' {
        true
    } else {
        false
    }
}

fn ascii_to_int(byte: u8) -> Option<u8> {
    if is_num(byte) {
        Some(byte - b'0')
    } else {
        None
    }
}
