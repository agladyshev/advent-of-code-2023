use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

pub fn part_one() -> Result<usize, std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let numbers = parse_line_numbers(&line);
        match numbers {
            Some(result) => {
                if times.is_empty() {
                    times = result;
                } else {
                    distances = result;
                }
            }
            None => {}
        }
    }
    let mut total_wins = 1;
    for (n, race_time) in times.iter().enumerate() {
        let mut wins = 0;
        let mut button_time = 0;
        let distance_goal = distances[n];
        loop {
            let distance = (race_time - button_time) * button_time;
            if distance > distance_goal {
                wins += 1;
                println!("Button: {}, Distance: {}", button_time, distance);
            } else if wins > 0 {
                total_wins *= wins;
                break;
            } else if button_time > *race_time {
                break;
            }
            button_time += 1;
        }
    }
    Ok(total_wins)
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
