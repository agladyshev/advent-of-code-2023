use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut histories: Vec<Vec<isize>> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let num_strs = line.split(" ");
        let mut seq: Vec<isize> = Vec::new();
        for str in num_strs {
            seq.push(str.parse::<isize>().expect("NaN"));
        }
        histories.push(seq);
    }
    println!("Part one: {}", part_one(&histories));
    println!("Part two: {}", part_two(&histories));
    Ok(())
}

fn part_one(histories: &Vec<Vec<isize>>) -> isize {
    let mut result: isize = 0;
    for seq in histories {
        let next = seq.last().expect("Not empty") + extrapolate_forward(&seq);
        result += next;
    }
    return result;
}

fn part_two(histories: &Vec<Vec<isize>>) -> isize {
    let mut result: isize = 0;
    for seq in histories {
        let reversed: Vec<isize> = seq.iter().rev().cloned().collect();
        let next = reversed.last().expect("Not empty") + extrapolate_forward(&reversed);
        result += next;
    }
    return result;
}

fn extrapolate_forward(seq: &Vec<isize>) -> isize {
    let mut diff_seq: Vec<isize> = Vec::new();
    let mut is_last = true;
    for i in 1..seq.len() {
        let diff = seq[i] - seq[i - 1];
        if diff != 0 {
            is_last = false;
        }
        diff_seq.push(diff);
    }
    if !is_last {
        let next_diff = extrapolate_forward(&diff_seq);
        return *diff_seq.last().expect("diff not empty") + next_diff;
    } else {
        return *diff_seq.last().expect("diff not empty");
    }
}
