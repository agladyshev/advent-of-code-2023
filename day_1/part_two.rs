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

fn main() -> std::io::Result<()> {
    let map: HashMap<_, _> = NUMBERS.iter().cloned().collect();
    for (key, value) in &map {
        println!("{} -> {}", key, value);
    }
    // const BUFFER_LEN:usize = 512;
    // let mut buffer = [0u8; BUFFER_LEN];
    let mut total: u16 = 0;
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut num_1 = None;
    let mut num_2 = None;
    for line in reader.lines() {
        let line = line?;
        for byte in line.bytes() {
            if is_num(byte) {
                if num_1.is_none() {
                    num_1 = Some(byte);
                }
                num_2 = Some(byte);
            }
        }
        total += (ascii_to_int(num_1.expect("num_1 is empty")).expect("num_1 is not a valid digit")
            * 10) as u16;
        total += (ascii_to_int(num_2.expect("num_2 is empty")).expect("num_2 is not a valid digit"))
            as u16;
        num_1 = None;
        num_2 = None;
    }
    println!("{}", total);
    Ok(())
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
