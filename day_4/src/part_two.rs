use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_two() -> std::io::Result<usize> {
    let mut i: usize = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut cards_to_read: Vec<usize> = Vec::new();
    let mut card_matches: HashMap<usize, usize> = HashMap::new();
    for (n, line_result) in reader.lines().enumerate() {
        cards_to_read.push(n);
        for card in &cards_to_read {
            //print!("{} ", card);
        }
        //println!("Cards to read {}", cards_to_read.len());
        match line_result {
            Ok(line) => {
                while i < cards_to_read.len() && cards_to_read[i] <= n {
                    //println!("i = {}, card num {}", i, cards_to_read[i]);
                    let card = cards_to_read[i];
                    let matches: usize;
                    if card_matches.contains_key(&card) {
                        matches = *card_matches.get(&card).unwrap();
                    } else {
                        matches = get_matches(&line);
                        card_matches.insert(card, matches);
                    }
                    //println!("Matches: {}", matches);
                    if matches != 0 {
                        for j in 0..matches {
                            //println!("Add card {}", card + j + 1);
                            cards_to_read.push(card + j + 1);
                        }
                    }
                    i += 1;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Ok(cards_to_read.len());
}

fn get_matches(line: &String) -> usize {
    let mut value = 0;
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
                winning_numbers.insert(number);
                number = 0;
            }
        }
        i += 1;
    }
    while i < bytes.len() {
        let byte = bytes[i];
        if byte >= b'0' && byte <= b'9' {
            number = number * 10 + (byte - b'0') as u32;
        } else {
            if number != 0 {
                if winning_numbers.contains(&number) {
                    value += 1;
                }
                number = 0;
            }
        }
        i += 1;
    }
    if number != 0 {
        if winning_numbers.contains(&number) {
            value += 1;
        }
    }
    return value;
}
