use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: HandType,
}

fn get_hand_type(cards: &str) -> HandType {
    // implement
    let mut card_freq: HashMap<char, u8> = HashMap::new();
    for card in cards.chars() {
        *card_freq.entry(card).or_insert(0) += 1;
    }
    let mut freq: Vec<&u8> = card_freq.values().collect();
    freq.sort_by(|a, b| b.cmp(a));
    match *freq[0] {
        5 => return HandType::FiveOfAKind,
        4 => return HandType::FourOfAKind,
        3 => {
            if *freq[1] == 2 {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }
        2 => {
            if *freq[1] == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        _ => return HandType::HighCard,
    }
}

pub fn part_one() -> Result<usize, std::io::Error> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = Vec::new();
    let mut hand_type_value: HashMap<HandType, u8> = HashMap::new();
    hand_type_value.insert(HandType::HighCard, 0);
    hand_type_value.insert(HandType::OnePair, 1);
    hand_type_value.insert(HandType::TwoPair, 2);
    hand_type_value.insert(HandType::ThreeOfAKind, 3);
    hand_type_value.insert(HandType::FullHouse, 4);
    hand_type_value.insert(HandType::FourOfAKind, 5);
    hand_type_value.insert(HandType::FiveOfAKind, 6);
    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let cards = parts[0];
        let bid = parts[1].parse::<usize>().expect("Invalid integer in bid");
        let hand_type = get_hand_type(cards);
        hands.push(Hand {
            cards: cards.to_string(),
            bid,
            hand_type,
        });
    }
    println!("{:?}", hands);
    Ok(0)
}
