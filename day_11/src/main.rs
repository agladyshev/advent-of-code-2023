use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

struct GameMap {
    map: Vec<Vec<bool>>,
    empty_x: Vec<bool>,
    empty_y: Vec<bool>,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let game_map = parse_map(&file).expect("Bad map");
    let x_additional_space: Vec<usize> = calc_modifier(&game_map.empty_x);
    let y_additional_space: Vec<usize> = calc_modifier(&game_map.empty_y);
    println!("Empty x: {:?}", &x_additional_space);
    println!("Empty y: {:?}", &y_additional_space);
    let mut y = 0;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut sum = 0;
    let mut pairs = 0;
    for row in game_map.map {
        let mut x = 0;
        for value in row {
            if value == true {
                let galaxy = Galaxy {
                    x: x + x_additional_space[x],
                    y: y + y_additional_space[y],
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
    println!("Pairs: {pairs}");
    println!("Sum: {sum}");
    Ok(())
}

fn get_distance(a: &Galaxy, b: &Galaxy) -> usize {
    let x_mod = if b.x > a.x { b.x - a.x } else { a.x - b.x };
    let y_mod = if b.y > a.y { b.y - a.y } else { a.y - b.y };
    return x_mod + y_mod;
}

fn calc_modifier(empty_coordinates: &Vec<bool>) -> Vec<usize> {
    let mut additional_space: Vec<usize> = Vec::new();
    let mut modifier = 0;
    for value in empty_coordinates {
        additional_space.push(modifier);
        if *value == true {
            modifier += 999999;
        }
    }
    return additional_space;
}

fn parse_map(file: &File) -> Option<GameMap> {
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut empty_x: Vec<bool> = Vec::new();
    let mut empty_y: Vec<bool> = Vec::new();
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result.expect("Bad line");
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
        empty_y.push(is_row_empty);
        map.push(row);
    }
    return Some(GameMap {
        map,
        empty_x,
        empty_y,
    });
}
