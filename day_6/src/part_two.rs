use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

pub fn part_two() -> Result<usize, std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut race_time: usize = 0;
    let mut distance_goal: usize = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let number = parse_line_number(&line);
        match number {
            Some(result) => {
                if race_time == 0 {
                    race_time = result;
                } else {
                    distance_goal = result;
                }
            }
            None => {}
        }
    }
    let mut first_win = 0;
    let mut last_win = race_time;
    println!("Race time: {}, distance goal: {}", race_time, distance_goal);
    loop {
        let distance = (race_time - first_win) * first_win;
        if distance > distance_goal {
            println!("Button: {}, Distance: {}", first_win, distance);
            break;
        } else if first_win > race_time {
            break;
        }
        first_win += 1;
    }
    loop {
        let distance = (race_time - last_win) * last_win;
        if distance > distance_goal {
            println!("Button: {}, Distance: {}", last_win, distance);
            break;
        } else if last_win == 0 {
            break;
        }
        last_win -= 1;
    }
    Ok(last_win - first_win + 1)
}

fn atoi(chars: &Vec<char>) -> Result<usize, ParseIntError> {
    return chars.iter().collect::<String>().parse::<usize>();
}

fn parse_line_number(line: &String) -> Option<usize> {
    let mut digits: Vec<char> = Vec::new();
    for char in line.chars() {
        if char.is_digit(10) {
            digits.push(char);
        }
    }
    if !digits.is_empty() {
        Some(atoi(&digits).expect("Invalid number"))
    } else {
        None
    }
}
