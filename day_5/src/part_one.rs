use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;

pub fn part_one() -> Result<usize, std::io::Error> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut seeds: Vec<usize> = Vec::new();
    let mut transform_table: Vec<(usize, usize, usize)> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let numbers = parse_line_numbers(&line);
        match numbers {
            Some(result) => {
                if n == 0 {
                    seeds = result;
                    println!("Seeds: ");
                    for seed in &seeds {
                        print!("{seed} ");
                    }
                    // destination, source, range
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
                    for seed in &mut seeds {
                        *seed = transform_value(&seed, &transform_table);
                    }
                    println!("Seeds: ");
                    for seed in &seeds {
                        print!("{seed} ");
                    }
                    transform_table.clear();
                }
            }
        }
    }
    if !transform_table.is_empty() {
        for seed in &mut seeds {
            *seed = transform_value(&seed, &transform_table);
        }
        println!("Seeds: ");
        for seed in &seeds {
            print!("{seed} ");
        }
        transform_table.clear();
    }
    seeds.sort();
    Ok(seeds[0])
}

fn transform_value(seed: &usize, transform_table: &Vec<(usize, usize, usize)>) -> usize {
    for item in transform_table {
        let source = item.1;
        let range = item.2;
        if *seed >= source && *seed < source + range {
            return item.0 + (*seed - source);
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
