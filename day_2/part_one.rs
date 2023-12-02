use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut result = 0;
    let mut bag = HashMap::new();
    bag.insert("red".to_string(), 12);
    bag.insert("green".to_string(), 13);
    bag.insert("blue".to_string(), 14);
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let game_data: Vec<&str> = line.split(":").collect();
        let game_number = get_int(game_data[0]).expect("No game number found");
        let rounds: Vec<&str> = game_data[1].split(";").collect();
        if is_valid_game(rounds, &bag) {
            result += game_number;
        }
    }
    println!("{}", result);
    Ok(())
}

fn is_valid_game(rounds: Vec<&str>, bag: &HashMap<String, u32>) -> bool {
    for round in rounds {
        let entries: Vec<&str> = round.split(",").collect();
        for entry in entries {
            let value = get_int(entry).expect("Missing cube number");
            let key = get_key(entry, bag).expect("Missing cube color");
            if value > *bag.get(&key).expect("Key not found") {
                return false;
            }
        }
    }
    true
}

fn get_key(str: &str, map: &HashMap<String, u32>) -> Option<String> {
    for key in map.keys() {
        if str.contains(key) {
            return Some(key.to_string());
        }
    }
    None
}

fn get_int(str: &str) -> Option<u32> {
    let mut digits = String::new();
    for char in str.chars() {
        if char.is_digit(10) {
            digits.push(char);
        }
    }
    if !digits.is_empty() {
        digits.parse::<u32>().ok()
    } else {
        None
    }
}
