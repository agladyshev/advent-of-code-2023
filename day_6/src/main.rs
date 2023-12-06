mod part_one;
mod part_two;
use part_one::part_one;
use part_two::part_two;

fn main() -> std::io::Result<()> {
    //println!("Result 1: {}", part_one()?);
    println!("Result 2: {}", part_two()?);
    Ok(())
}
