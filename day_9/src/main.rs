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
    println!("{:?}", histories);
    Ok(())
}
