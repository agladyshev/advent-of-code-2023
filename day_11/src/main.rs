use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut empty_x: Vec<bool> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if empty_x.is_empty() {
            for _i in 0..line.len() {
                empty_x.push(true);
            }
        }
        let mut row = Vec::new();
        let mut is_row_empty = true;
        for (x, char) in line.chars().enumerate() {
            let is_galaxy = char == '#';
            if is_galaxy {
                empty_x[x] = false;
                is_row_empty = false;
            }
            row.push(is_galaxy);
        }
        if is_row_empty {
            map.push(row.clone());
        }
        map.push(row);
    }
    let mut x_additional_space: Vec<usize> = Vec::new();
    let mut modifier = 0;
    for value in &empty_x {
        x_additional_space.push(modifier);
        if *value == true {
            modifier += 1;
        }
    }
    println!("Empty x: {:?}", &empty_x);
    println!("Empty x: {:?}", &x_additional_space);
    let mut y = 0;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut sum = 0;
    let mut pairs = 0;
    for row in map {
        let mut x = 0;
        for value in row {
            if value == true {
                let galaxy = Galaxy {
                    x: x + x_additional_space[x],
                    y,
                };
                for prev in &galaxies {
                    let distance = get_distance(&prev, &galaxy);
                    //println!("{distance}");
                    pairs += 1;
                    sum += distance;
                }
                galaxies.push(galaxy);
            }
            x += 1;
        }
        y += 1;
    }
    //println!("{:?}", galaxies);
    println!("{}", galaxies.len());
    println!("Sum: {sum}");
    println!("Pairs: {pairs}");
    Ok(())
}

fn get_distance(a: &Galaxy, b: &Galaxy) -> usize {
    //println!("a: {:?}, b: {:?}", a, b);
    let x_mod = if b.x > a.x { b.x - a.x } else { a.x - b.x };
    let y_mod = if b.y > a.y { b.y - a.y } else { a.y - b.y };
    return x_mod + y_mod;
}
