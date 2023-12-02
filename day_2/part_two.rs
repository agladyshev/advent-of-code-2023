use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut result = 0;
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let game_data: Vec<&str> = line.split(":").collect();
        let rounds: Vec<&str> = game_data[1].split(";").collect();
        result += get_power(rounds);
    }
    println!("{}", result);
    Ok(())
}

fn get_power(rounds: Vec<&str>) -> u32 {
    let mut bag = HashMap::new();
    let mut power = 1;
    bag.insert("red".to_string(), 0);
    bag.insert("green".to_string(), 0);
    bag.insert("blue".to_string(), 0);
    for round in rounds {
        let entries: Vec<&str> = round.split(",").collect();
        for entry in entries {
            let value = get_int(entry).expect("Missing cube number");
            let key = get_key(entry, &bag).expect("Missing cube color");
            if value > *bag.get(&key).expect("Key not found") {
                bag.insert(key, value);
            }
        }
    }
    for value in bag.values() {
        power *= value;
    }
    return power;
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
