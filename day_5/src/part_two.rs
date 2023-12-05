use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;

pub fn part_two() -> Result<usize, std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut seed_pairs: Vec<(usize, usize)> = Vec::new();
    let mut transform_tables: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    let mut transform_table: Vec<(usize, usize, usize)> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let numbers = parse_line_numbers(&line);
        match numbers {
            Some(result) => {
                if n == 0 {
                    for i in 0..(result.len() / 2) {
                        let index = i * 2;
                        seed_pairs.push((result[index], result[index + 1]));
                    }
                } else {
                    if result.len() == 3 {
                        transform_table.push((result[0], result[1], result[2]));
                    } else {
                        return Err(Error::new(ErrorKind::Other, "oh no!"));
                    }
                }
            }
            None => {
                if !transform_table.is_empty() {
                    transform_tables.push(transform_table.clone());
                    transform_table.clear();
                }
            }
        }
    }
    if !transform_table.is_empty() {
        transform_tables.push(transform_table.clone());
        transform_table.clear();
    }
    let mut seed = 0;
    loop {
        let mut result = seed;
        for table in transform_tables.iter().rev() {
            result = rev_transform_value(&result, &table);
        }
        //println!("{result}");
        for pair in &seed_pairs {
            //println!("{} {}", pair.0, pair.1);
            if result >= pair.0 && result < (pair.0 + pair.1) {
                return Ok(seed);
            }
        }
        seed += 1;
    }
}

fn rev_transform_value(seed: &usize, transform_table: &Vec<(usize, usize, usize)>) -> usize {
    for item in transform_table {
        let source = item.1;
        let range = item.2;
        let dest = item.0;
        if *seed >= dest && *seed < dest + range {
            return source + (*seed - dest);
        }
    }
    return *seed;
}

fn atoi(chars: &Vec<char>) -> Result<usize, ParseIntError> {
    return chars.iter().collect::<String>().parse::<usize>();
}

fn parse_line_numbers(line: &String) -> Option<Vec<usize>> {
    let mut numbers: Vec<usize> = Vec::new();
    let mut digits: Vec<char> = Vec::new();
    for char in line.chars() {
        if char.is_digit(10) {
            digits.push(char);
        } else {
            if !digits.is_empty() {
                numbers.push(atoi(&digits).expect("Invalid number"));
            }
            digits.clear();
        }
    }
    if !digits.is_empty() {
        numbers.push(atoi(&digits).expect("Invalid number"));
    }
    if numbers.len() != 0 {
        Some(numbers)
    } else {
        None
    }
}
