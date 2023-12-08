mod part_one;
//mod part_two;
use part_one::part_one;
//use part_two::part_two;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    // let 1 == R, 0 == L
    let mut actions: Vec<bool> = Vec::new();
    for (n, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if n == 0 {
            for char in line.chars() {
                if char == 'R' {
                    actions.push(true);
                } else if char == 'L' {
                    actions.push(false);
                }
            }
        } else if n == 1 {
            continue;
        } else {
            let parts: Vec<&str> = line.split(" = ").collect();
            assert_eq!(parts.len(), 2);
            let key = parts[0];
            let tuple_str = parts[1];
            let tuple_str_trimmed = tuple_str.trim_matches(|c| c == '(' || c == ')');
            let tuple_parts: Vec<&str> = tuple_str_trimmed.split(", ").collect();
            assert_eq!(parts.len(), 2);
            let options: (String, String) =
                (tuple_parts[0].to_string(), tuple_parts[1].to_string());
            map.insert(key.to_string(), options);
        }
    }
    println!("{:?}", actions);
    println!("{:?}", map);
    //println!("Result 1: {}", part_one()?);
    Ok(())
}
