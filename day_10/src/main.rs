use std::fs::File;
use std::io::{BufRead, BufReader};

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
    match animal_location {
        Some(location) => {
            println!("Animal location, x: {}, y: {}", location.0, location.1);
        }
        None => {}
    }
    Ok(())
}
