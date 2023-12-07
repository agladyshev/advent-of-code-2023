use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPai,
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
    HandType::HighCard
}

pub fn part_one() -> Result<usize, std::io::Error> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = Vec::new();
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
