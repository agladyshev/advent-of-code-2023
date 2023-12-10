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
    let mut x_len = 0;
    let mut y_len = 0;
    for (y, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if y == 0 {
            x_len = line.len();
        }
        let mut chars: Vec<char> = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            if tile == 'S' {
                animal_location = Some((x, y));
            }
            chars.push(tile);
        }
        game_map.push(chars);
    }
    y_len = game_map.len();
    match animal_location {
        Some(location) => {
            println!("Animal location, x: {}, y: {}", location.0, location.1);
            if location.0 > 0 {
                let west_tile = game_map[location.1][location.0 - 1];
                if WEST_VALID.contains(&west_tile) {
                    println!("West: {west_tile}");
                    traverse((location.0 - 1, location.1), Direction::West, &game_map);
                    return Ok(());
                }
            }
            if location.0 < x_len {
                let east_tile = game_map[location.1][location.0 + 1];
                if EAST_VALID.contains(&east_tile) {
                    println!("East: {east_tile}");
                    traverse((location.0 + 1, location.1), Direction::East, &game_map);
                    return Ok(());
                }
            }
            if location.1 > 0 {
                let north_tile = game_map[location.1 - 1][location.0];
                if NORTH_VALID.contains(&north_tile) {
                    println!("North: {north_tile}");
                    traverse((location.0, location.1 - 1), Direction::North, &game_map);
                    return Ok(());
                }
            }
            if location.1 < y_len {
                let south_tile = game_map[location.1 + 1][location.0];
                if SOUTH_VALID.contains(&south_tile) {
                    println!("South: {south_tile}");
                    traverse((location.0, location.1 + 1), Direction::South, &game_map);
                    return Ok(());
                }
            }
        }
        None => {}
    }
    Ok(())
}

fn traverse(location: (usize, usize), direction: Direction, map: &Vec<Vec<char>>) -> usize {
    let mut next_position = (location, direction);
    let mut steps = 1;
    loop {
        let current_location = next_position.0;
        let tile = map[current_location.1][current_location.0];
        next_position = go(current_location, tile, next_position.1);
        println!("{:?}", next_position);
        steps += 1;
        let x = next_position.0 .0;
        let y = next_position.0 .1;
        if map[y][x] == 'S' {
            break;
        }
        if steps > 100 {
            // break;
        }
    }
    let distance = steps / 2;
    println!("Longest from start: {distance}");
    return distance;
}

fn go(
    location: (usize, usize),
    pipe_type: char,
    direction: Direction,
) -> ((usize, usize), Direction) {
    println!("Symbol: {pipe_type}");
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
