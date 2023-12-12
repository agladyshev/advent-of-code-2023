use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum: usize = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
        let mut line_parts = line.split(" ");
        let springs: Vec<char> = line_parts.next().unwrap().chars().collect();
        let numbers: Vec<usize> = line_parts
            .next()
            .unwrap()
            .split(",")
            .filter_map(|str| str.parse::<usize>().ok())
            .collect();
        let mut multiplied_springs: Vec<char> = Vec::new();
        let multiplied_numbers: Vec<usize> = numbers
            .iter()
            .cycle()
            .take(numbers.len() * 5)
            .cloned()
            .collect();
        for i in 0..5 {
            for char in &springs {
                multiplied_springs.push(*char);
            }
            if i != 4 {
                multiplied_springs.push('?');
            }
        }
        //println!("{:?}", multiplied_springs);
        //println!("{:?}", multiplied_numbers);
        let len = multiplied_springs.len();
        let count: usize = get_combinations_count(
            multiplied_springs,
            &multiplied_numbers,
            0,
            len,
            Vec::new(),
            0,
        );
        println!("Line combinations: {count}");
        sum += count;
    }
    println!("Sum of combinations: {sum}");
    Ok(())
}

fn get_combinations_count(
    chars: Vec<char>,
    pattern: &Vec<usize>,
    start: usize,
    len: usize,
    arrangement: Vec<usize>,
    count: usize,
) -> usize {
    let mut base_arrangement = arrangement.clone();
    let mut base_count = count;
    for i in start..len {
        //println!("{i} of {len}");
        if chars[i] == '?' {
            let mut option_one = chars.clone();
            option_one[i] = '#';
            let mut option_two = chars.clone();
            option_two[i] = '.';
            let mut arrangement_two = base_arrangement.clone();
            if base_count != 0 {
                arrangement_two.push(base_count);
            }
            let is_one_valid_path = pre_check_arrangement(&base_arrangement, &pattern);
            let is_two_valid_path = pre_check_arrangement(&arrangement_two, &pattern);
            let mut sum = 0;
            if is_one_valid_path {
                sum += get_combinations_count(
                    option_one,
                    pattern,
                    i + 1,
                    len,
                    base_arrangement,
                    base_count + 1,
                )
            }
            if is_two_valid_path {
                sum += get_combinations_count(option_two, pattern, i + 1, len, arrangement_two, 0);
            }
            return sum;
        } else {
            if chars[i] == '#' {
                base_count += 1;
            } else {
                if base_count != 0 {
                    base_arrangement.push(base_count);
                }
                base_count = 0;
            }
        }
    }
    //println!("{:?}", chars);
    if base_count != 0 {
        base_arrangement.push(base_count);
    }
    //check_pattern(&chars, pattern);
    //if check_pattern(&chars, pattern) {
    if check_arrangement(&base_arrangement, pattern) {
        return 1;
    } else {
        return 0;
    }
}

fn pre_check_arrangement(arrangement: &Vec<usize>, pattern: &Vec<usize>) -> bool {
    let length = arrangement.len();
    let pattern_len = pattern.len();
    if pattern_len < length {
        return false;
    }
    for i in 0..length {
        if i == (length - 1) {
            return arrangement[i] <= pattern[i];
        } else if arrangement[i] != pattern[i] {
            return false;
        }
    }
    return true;
}

fn check_arrangement(arrangement: &Vec<usize>, pattern: &Vec<usize>) -> bool {
    //println!("New: {:?}", arrangement);
    if arrangement.len() == pattern.len() {
        for i in 0..arrangement.len() {
            if arrangement[i] != pattern[i] {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn check_pattern(chars: &Vec<char>, pattern: &Vec<usize>) -> bool {
    let mut arrangement: Vec<usize> = Vec::new();
    let mut count = 0;
    for char in chars {
        if *char == '#' {
            count += 1;
        } else {
            if count != 0 {
                arrangement.push(count);
            }
            count = 0;
        }
    }
    if count != 0 {
        arrangement.push(count);
    }
    println!("Old {:?}", arrangement);
    if arrangement.len() == pattern.len() {
        for i in 0..arrangement.len() {
            if arrangement[i] != pattern[i] {
                return false;
            }
        }
        return true;
    }
    return false;
}
