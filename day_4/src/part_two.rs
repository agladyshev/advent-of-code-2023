use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_two() -> std::io::Result<u32> {
    let mut sum = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let matches = get_matches(&line);
        if matches.is_some() {
            sum += matches.unwrap();
            //println!("{}", value.unwrap());
        }
    }

    return Ok(sum);
}

fn get_matches(line: &String) -> Option<u32> {
    let mut value: Option<u32> = None;
    let bytes = line.as_bytes();
    let mut winning_numbers: HashSet<u32> = HashSet::new();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b':' {
        i += 1;
    }
    let mut number: u32 = 0;
    while i < bytes.len() && bytes[i] != b'|' {
        let byte = bytes[i];
        if byte >= b'0' && byte <= b'9' {
            number = number * 10 + (byte - b'0') as u32;
        } else {
            if number != 0 {
                //print!("{} ", number);
                winning_numbers.insert(number);
                number = 0;
            }
        }
        i += 1;
    }
    while i < bytes.len() {
        let byte = bytes[i];
        //print!("{} ", byte as char);
        if byte >= b'0' && byte <= b'9' {
            number = number * 10 + (byte - b'0') as u32;
        } else {
            if number != 0 {
                //print!("{} ", number);
                if winning_numbers.contains(&number) {
                    if value.is_none() {
                        value = Some(1);
                    } else {
                        value = Some(value.unwrap() * 2);
                    }
                }
                number = 0;
            }
        }
        i += 1;
    }
    if number != 0 {
        //print!("{} ", number);
        if winning_numbers.contains(&number) {
            if value.is_none() {
                value = Some(1);
            } else {
                value = Some(value.unwrap() * 2);
            }
        }
    }
    return value;
}
