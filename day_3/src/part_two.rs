use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct PartNumber {
    number: u32,
    index: usize,
    len: usize,
}

pub fn part_two() -> std::io::Result<()> {
    let mut part_sum = 0;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // init stack of size 3
    let mut numbers_stack: VecDeque<Vec<PartNumber>> = VecDeque::new();
    numbers_stack.push_back(Vec::new());
    numbers_stack.push_back(Vec::new());
    numbers_stack.push_back(Vec::new());
    let mut previous_symbols: Vec<usize> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut symbols: Vec<usize> = Vec::new();
        let mut digits = String::new();
        let mut digit_start = None;
        let mut new_numbers: Vec<PartNumber> = Vec::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if digit_start.is_none() {
                    digit_start = Some(i);
                }
                digits.push(char);
            } else {
                if digits.len() != 0 {
                    new_numbers.push(PartNumber {
                        number: digits.parse::<u32>().expect("Not a number"),
                        index: digit_start.expect("Invalid index"),
                        len: digits.len(),
                    });
                    digit_start = None;
                    digits.clear();
                }
                if char == '*' {
                    symbols.push(i);
                }
            }
        }
        if digits.len() != 0 {
            new_numbers.push(PartNumber {
                number: digits.parse::<u32>().expect("Not a number"),
                index: digit_start.expect("Invalid index"),
                len: digits.len(),
            });
        }
        // update stack
        numbers_stack.pop_front();
        numbers_stack.push_back(new_numbers);
        // Check last numbers
        let current_numbers: Vec<PartNumber> = get_current_numbers(&numbers_stack);
        if previous_symbols.len() != 0 && current_numbers.len() != 0 {
            for symbol in &previous_symbols {
                if let Some(value) = get_symbol_value(&symbol, &current_numbers) {
                    part_sum += value;
                }
            }
        }
        print_line(&previous_symbols, &numbers_stack);
        previous_symbols = symbols;
    }
    numbers_stack.pop_front();
    let current_numbers: Vec<PartNumber> = get_current_numbers(&numbers_stack);
    if previous_symbols.len() != 0 && current_numbers.len() != 0 {
        for symbol in &previous_symbols {
            if let Some(value) = get_symbol_value(&symbol, &current_numbers) {
                part_sum += value;
            }
        }
    }
    print_line(&previous_symbols, &numbers_stack);
    println!("\n{}", part_sum);
    Ok(())
}

fn get_symbol_value(symbol: &usize, numbers: &Vec<PartNumber>) -> Option<u32> {
    let mut value = 1;
    let mut count = 0;
    for number in numbers {
        let mut start = 0;
        let end;
        if number.index == 0 {
            end = number.len;
        } else {
            start = number.index - 1;
            end = start + number.len + 1;
        }
        // 123 at 0. start 0, end 3
        // 123 at 1, start 0, end 4
        if *symbol >= start && *symbol <= end {
            count += 1;
            if count > 2 {
                return None;
            }
            value = value * number.number;
        }
    }
    if count != 2 {
        return None;
    }
    return Some(value);
}

fn get_current_numbers(numbers_stack: &VecDeque<Vec<PartNumber>>) -> Vec<PartNumber> {
    let mut result = Vec::new();
    for part_vec in numbers_stack {
        result.extend(part_vec.iter().cloned());
    }
    result
}

fn print_line(previous_symbols: &Vec<usize>, numbers_stack: &VecDeque<Vec<PartNumber>>) {
    let mut line: Vec<char> = vec!['.'; 140];
    for symbol in previous_symbols {
        let success = get_symbol_value(symbol, &get_current_numbers(numbers_stack)).is_some();
        if success {
            line[*symbol] = 'V';
        }
    }
    for part in &numbers_stack[1] {
        for i in part.index..(part.index + part.len) {
            line[i] = 'x';
        }
    }
    let line_string: String = line.iter().collect();
    println!("{}", line_string);
}
