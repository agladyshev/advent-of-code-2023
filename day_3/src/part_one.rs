use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PartNumber {
    number: u32,
    index: usize,
    len: usize,
}

pub fn part_one() -> std::io::Result<()> {
    let mut part_sum = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // init stack of size 3
    let mut symbol_stack: VecDeque<Vec<usize>> = VecDeque::new();
    symbol_stack.push_back(Vec::new());
    symbol_stack.push_back(Vec::new());
    symbol_stack.push_back(Vec::new());
    let mut previous_parts: Vec<PartNumber> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut symbols = Vec::new();
        let mut digits = String::new();
        let mut digit_start = None;
        let mut current_parts: Vec<PartNumber> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if digit_start.is_none() {
                    digit_start = Some(i);
                }
                digits.push(char);
            } else {
                if digits.len() != 0 {
                    current_parts.push(PartNumber {
                        number: digits.parse::<u32>().expect("Not a number"),
                        index: digit_start.expect("Invalid index"),
                        len: digits.len(),
                    });
                    digit_start = None;
                    digits.clear();
                }
                if char != '.' {
                    symbols.push(i);
                }
            }
        }
        if digits.len() != 0 {
            current_parts.push(PartNumber {
                number: digits.parse::<u32>().expect("Not a number"),
                index: digit_start.expect("Invalid index"),
                len: digits.len(),
            });
        }
        // update stack
        symbol_stack.pop_front();
        symbol_stack.push_back(symbols.clone());
        let current_symbols: HashSet<usize> = get_current_symbols(&symbol_stack);
        // Check last numbers
        if previous_parts.len() != 0 && current_symbols.len() != 0 {
            for part in &previous_parts {
                if check_part(part, &current_symbols) {
                    part_sum += part.number;
                }
            }
        }
        print_line(&previous_parts, &symbol_stack);
        previous_parts = current_parts;
    }
    symbol_stack.pop_front();
    let current_symbols: HashSet<usize> = get_current_symbols(&symbol_stack);
    // Check last numbers
    for part in &previous_parts {
        if check_part(part, &current_symbols) {
            part_sum += part.number;
        }
    }
    print_line(&previous_parts, &symbol_stack);
    println!("\n{}", part_sum);
    Ok(())
}

fn check_part(part: &PartNumber, symbols: &HashSet<usize>) -> bool {
    let mut start = 0;
    let mut end = 0;
    if part.index != 0 {
        start = part.index - 1;
        end += 1;
    }
    end += start + part.len;
    while start <= end {
        if symbols.contains(&start) {
            return true;
        }
        start += 1;
    }
    //for &index in symbols {
    //    if index >= start && index <= end {
    //        return true;
    //    }
    //}
    return false;
}

fn get_current_symbols(symbol_stack: &VecDeque<Vec<usize>>) -> HashSet<usize> {
    let mut symbols = HashSet::new();
    for vec in symbol_stack {
        for index in vec {
            symbols.insert(*index);
        }
    }
    symbols
}

fn print_line(previous_parts: &Vec<PartNumber>, symbol_stack: &VecDeque<Vec<usize>>) {
    let mut line: Vec<char> = vec!['.'; 140];
    for part in previous_parts {
        let success = check_part(part, &get_current_symbols(symbol_stack));
        for i in part.index..(part.index + part.len) {
            if success {
                line[i] = 'V';
            } else {
                line[i] = 'X';
            }
        }
    }
    for &i in &symbol_stack[1] {
        line[i] = '*';
    }
    let line_string: String = line.iter().collect();
    println!("{}", line_string);
}
