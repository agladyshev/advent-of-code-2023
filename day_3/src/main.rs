use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PartNumber {
    number: u32,
    index: usize,
}

fn main() -> std::io::Result<()> {
    // line: numbers[] - index + number
    // line: symbols[] - index
    // for each line get symbols for prevline, current, next
    // for each number in line check if symbols are adjacent
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // init stack of size 3
    let mut symbol_stack: VecDeque<Vec<usize>> = VecDeque::new();
    symbol_stack.push_back(Vec::new());
    symbol_stack.push_back(Vec::new());
    symbol_stack.push_back(Vec::new());
    let mut last_numbers: Vec<PartNumber> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut symbols = Vec::new();
        let mut digits = String::new();
        let mut digit_start = 0;
        let mut current_numbers: Vec<PartNumber> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if digit_start == 0 {
                    digit_start = i;
                }
                digits.push(char);
            } else {
                if digits.len() != 0 {
                    current_numbers.push(PartNumber {
                        number: digits.parse::<u32>().expect("Not a number"),
                        index: digit_start,
                    });
                    digit_start = 0;
                    digits.clear();
                }
                if char != '.' {
                    symbols.push(i);
                }
            }
        }
        if digits.len() != 0 {
            current_numbers.push(PartNumber {
                number: digits.parse::<u32>().expect("Not a number"),
                index: digit_start,
            });
        }
        // check current numbers

        //update stack
        symbol_stack.pop_front();
        symbol_stack.push_back(symbols.clone());
        let mut current_symbols: HashSet<usize> = HashSet::new();
        for vec in &symbol_stack {
            for index in vec {
                current_symbols.insert(*index);
            }
        }
        if last_numbers.len() != 0 {
            print!("Numbers to check: ");
            for part in last_numbers {
                print!("{}:{} ", part.index, part.number);
            }
        }
        print!("\nCurrent symbols ");
        for i in symbols {
            print!("{} ", i);
        }
        print!("\nIn stack: ");
        if current_symbols.len() == 0 {
            print!("empty");
        } else {
            for i in current_symbols {
                print!("{} ", i);
            }
        }
        print!("\nCurrent numbers: ");
        for part in &current_numbers {
            print!("{}:{} ", part.index, part.number);
        }
        println!("\n");
        last_numbers = current_numbers;
    }
    if last_numbers.len() != 0 {
        print!("Numbers to check: ");
        for part in last_numbers {
            print!("{}:{} ", part.index, part.number);
        }
    }
    println!("\n");
    Ok(())
}
