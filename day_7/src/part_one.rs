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

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}

#[derive(Debug, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Ace => 13,
            Card::King => 12,
            Card::Queen => 11,
            Card::Jack => 10,
            Card::Ten => 9,
            Card::Nine => 8,
            Card::Eight => 7,
            Card::Seven => 6,
            Card::Six => 5,
            Card::Five => 4,
            Card::Four => 3,
            Card::Three => 2,
            Card::Two => 1,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

fn card_by_char(char: &char) -> Card {
    match char {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        _ => Card::Two,
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
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
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let cards_str = parts[0];
        let mut cards: Vec<Card> = Vec::new();
        for char in cards_str.chars() {
            cards.push(card_by_char(&char));
        }
        let bid = parts[1].parse::<usize>().expect("Invalid integer in bid");
        let hand_type = get_hand_type(&cards_str);
        hands.push(Hand {
            cards,
            bid,
            hand_type,
        });
    }
    hands.sort_by(compare_hands);
    let mut acc = 0;
    let mut rank = 1;
    for hand in hands {
        acc += hand.bid * rank;
        rank += 1;
        println!("{:?}", hand);
    }
    Ok(acc)
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    a.hand_type
        .value()
        .cmp(&b.hand_type.value())
        .then_with(|| a.cards.cmp(&b.cards))
}
