use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WEST_VALID: [char; 3] = ['-', 'F', 'L'];
const EAST_VALID: [char; 3] = ['-', 'J', '7'];
const SOUTH_VALID: [char; 3] = ['|', 'J', 'L'];
const NORTH_VALID: [char; 3] = ['|', 'F', '7'];

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    West,
    South,
    North,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut game_map: Vec<Vec<char>> = Vec::new();
    let mut animal_location: Option<(usize, usize)> = None;
    for (y, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let mut chars: Vec<char> = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            if tile == 'S' {
                animal_location = Some((x, y));
            }
            chars.push(tile);
        }
        game_map.push(chars);
    }
    let pipe_loop: Vec<(usize, usize)> =
        find_loop(animal_location.expect("No animal found"), &game_map).unwrap();
    //println!("{:?}", pipe_loop);
    println!(
        "First: {:?}, Last: {:?}",
        pipe_loop.first().unwrap(),
        pipe_loop.last().unwrap()
    );
    println!("Max distance: {}", &pipe_loop.len() / 2);
    //let mut loop_hashset = HashSet::new();
    //for e in pipe_loop {
    //    loop_hashset.insert(e);
    //}
    //let mut enclosed_tiles = 0;

    let area = calculate_area(&pipe_loop);
    let enclosed = (area + 1.0) - (pipe_loop.len() as f64) / 2.0;
    println!("{}", enclosed);
    Ok(())
}

fn calculate_area(points: &Vec<(usize, usize)>) -> f64 {
    let mut area: f64 = 0.0;
    let length = points.len();
    for i in 0..points.len() {
        let next = (i + 1) % length;
        area += (points[i].0 as f64) * (points[next].1 as f64);
        area -= (points[i].1 as f64) * (points[next].0 as f64);
    }
    (area as f64).abs() / 2.0
}

fn find_loop(location: (usize, usize), game_map: &Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let y_len = game_map.len();
    let x_len = game_map[0].len();
    //println!("Animal location, x: {}, y: {}", location.0, location.1);
    if location.0 > 0 {
        let west_tile = game_map[location.1][location.0 - 1];
        if WEST_VALID.contains(&west_tile) {
            println!("West: {west_tile}");
            return Some(traverse(
                (location.0 - 1, location.1),
                Direction::West,
                &game_map,
            ));
        }
    }
    if location.0 < x_len {
        let east_tile = game_map[location.1][location.0 + 1];
        if EAST_VALID.contains(&east_tile) {
            println!("East: {east_tile}");
            return Some(traverse(
                (location.0 + 1, location.1),
                Direction::East,
                &game_map,
            ));
        }
    }
    if location.1 > 0 {
        let north_tile = game_map[location.1 - 1][location.0];
        if NORTH_VALID.contains(&north_tile) {
            println!("North: {north_tile}");
            return Some(traverse(
                (location.0, location.1 - 1),
                Direction::North,
                &game_map,
            ));
        }
    }
    if location.1 < y_len {
        let south_tile = game_map[location.1 + 1][location.0];
        if SOUTH_VALID.contains(&south_tile) {
            println!("South: {south_tile}");
            return Some(traverse(
                (location.0, location.1 + 1),
                Direction::South,
                &game_map,
            ));
        }
    }
    None
}

fn traverse(
    location: (usize, usize),
    direction: Direction,
    map: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let mut next_position = (location, direction);
    let mut pipe_loop: Vec<(usize, usize)> = Vec::new();
    pipe_loop.push(location);
    loop {
        let current_location = next_position.0;
        let tile = map[current_location.1][current_location.0];
        next_position = go(current_location, tile, next_position.1);
        //println!("{:?}", next_position);
        let x = next_position.0 .0;
        let y = next_position.0 .1;
        pipe_loop.push(next_position.0);
        if map[y][x] == 'S' {
            break;
        }
    }
    return pipe_loop;
}

fn go(
    location: (usize, usize),
    pipe_type: char,
    direction: Direction,
) -> ((usize, usize), Direction) {
    //println!("Symbol: {pipe_type}");
    //println!("Direction: {:?}", direction);
    match pipe_type {
        '|' => {
            if direction == Direction::South {
                return ((location.0, location.1 + 1), direction);
            } else {
                return ((location.0, location.1 - 1), direction);
            }
        }
        '-' => {
            if direction == Direction::West {
                return ((location.0 - 1, location.1), direction);
            } else {
                return ((location.0 + 1, location.1), direction);
            }
        }
        'F' => {
            if direction == Direction::West {
                return ((location.0, location.1 + 1), Direction::South);
            } else {
                return ((location.0 + 1, location.1), Direction::East);
            }
        }
        '7' => {
            if direction == Direction::East {
                return ((location.0, location.1 + 1), Direction::South);
            } else {
                return ((location.0 - 1, location.1), Direction::West);
            }
        }
        'J' => {
            if direction == Direction::South {
                return ((location.0 - 1, location.1), Direction::West);
            } else {
                return ((location.0, location.1 - 1), Direction::North);
            }
        }
        'L' => {
            if direction == Direction::South {
                return ((location.0 + 1, location.1), Direction::East);
            } else {
                return ((location.0, location.1 - 1), Direction::North);
            }
        }
        _ => {
            return (location, direction);
        }
    }
}
