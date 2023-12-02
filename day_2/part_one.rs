use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut bag = HashMap::new();
    bag.insert("red", 12);
    bag.insert("green", 13);
    bag.insert("blue", 14);
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let game_data: Vec<&str> = line.split(":").collect();
        let game_number = getInt(game_data[0]).expect("No game number found");
        let rounds: Vec<&str> = game_data[1].split(";").collect();
        for round in rounds {
            let entries: Vec<&str> = round.split(",").collect();
            for entry in entries {
                let value = getInt(entry).expect("Missing cube number");
                let key = getKey(entry, &bag).expect("Missing cube color");
                println!("{} {}", key, value);
            }
        }
        break;
    }
    Ok(())
}

fn getKey(str: &str, map: &HashMap<&str, u32>) -> Option<String> {
    for key in map.keys() {
        if str.contains(key) {
            return Some(key.to_string());
        }
    }
    None
}

fn getInt(str: &str) -> Option<u32> {
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
