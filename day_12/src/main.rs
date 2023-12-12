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
        //println!("{:?}", springs);
        //println!("{:?}", numbers);
        let count: usize = get_combinations_count(springs, &numbers);
        //println!("Line combinations: {count}");
        sum += count;
    }
    println!("Sum of combinations: {sum}");
    Ok(())
}

fn get_combinations_count(chars: Vec<char>, pattern: &Vec<usize>) -> usize {
    let mut i: usize = 0;
    for char in &chars {
        if *char == '?' {
            let mut option_one = chars.clone();
            option_one[i] = '#';
            let mut option_two = chars.clone();
            option_two[i] = '.';
            //println!("{:?}", option_one);
            //println!("{:?}", option_two);
            return get_combinations_count(option_one, pattern)
                + get_combinations_count(option_two, pattern);
        }
        i += 1;
    }
    //println!("{:?}", chars);
    if check_pattern(&chars, pattern) {
        return 1;
    } else {
        return 0;
    }
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
    //println!("{:?}", arrangement);
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
