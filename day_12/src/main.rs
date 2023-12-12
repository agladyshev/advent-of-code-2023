use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum_pt_1: usize = 0;
    let mut sum_pt_2: usize = 0;
    let mut cache = HashMap::new();
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
        cache.clear();
        let count_pt_1: usize = combinations(&mut cache, &springs, None, &numbers);
        cache.clear();
        let count_pt_2: usize =
            combinations(&mut cache, &multiplied_springs, None, &multiplied_numbers);
        sum_pt_1 += count_pt_1;
        sum_pt_2 += count_pt_2;
    }
    println!("Sum of combinations, part 1: {sum_pt_1}");
    println!("Sum of combinations, part 2: {sum_pt_2}");
    Ok(())
}

fn combinations(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    substring: &[char],
    stash: Option<usize>,
    patterns: &[usize],
) -> usize {
    // This is a dumbified version of a solution below:
    // // https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs
    // patterns is stretches of ### to check
    // stash is the number of #
    if substring.is_empty() {
        return match (stash, patterns.len()) {
            //  succesfully traversed the entire string
            // stash is empty (previous char was .)
            (None, 0) => 1,
            // stash is not empty, check if it matches current pattern
            (Some(x), 1) if x == patterns[0] => 1,
            // else invalid combination
            _ => 0,
        };
    }
    if stash.is_some() && patterns.is_empty() {
        return 0;
    }

    let key = (substring.len(), stash.unwrap_or(0), patterns.len());
    // 19, 1, 4 - so key is a point in execution
    // if we start from this point we always get the same result
    // because it is for the same string

    if let Some(&cached_value) = cache.get(&key) {
        return cached_value;
    }

    let ways = match (substring[0], stash) {
        ('.', Some(x)) => {
            // We have some ### in stash, but find .
            if x != patterns[0] {
                // if number of ### does not equal the pattern, there is no match
                return 0;
            } else {
                // continue to the next part of the pattern with empty stash, continue to next char
                return combinations(cache, &substring[1..], None, &patterns[1..]);
            }
        }
        ('.', None) => {
            // if . but stash is empty, basically skip character
            return combinations(cache, &substring[1..], None, patterns);
        }
        ('#', Some(_)) => {
            // Found #, increase stash, next char
            return combinations(cache, &substring[1..], stash.map(|x| x + 1), patterns);
        }
        ('#', None) => {
            // Found #, create stash with one #, go to next character
            return combinations(cache, &substring[1..], Some(1), patterns);
        }
        ('?', Some(x)) => {
            // Branching out
            // If char is #, increase stash and get options for this branch
            let branch_hash = combinations(cache, &substring[1..], stash.map(|x| x + 1), patterns);
            // If char i ., check stash against the pattern, if matches, calculate this branch
            if x == patterns[0] {
                // empty stash, get next pattern
                let branch_dot = combinations(cache, &substring[1..], None, &patterns[1..]);
                branch_hash + branch_dot
            } else {
                branch_hash
            }
        }
        ('?', None) => {
            // First char is ?, nothing is determined yet, just split execution
            // if #, increase the stash
            return combinations(cache, &substring[1..], Some(1), patterns)
                + combinations(cache, &substring[1..], None, patterns);
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}
