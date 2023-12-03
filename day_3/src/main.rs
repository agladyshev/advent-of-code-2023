mod part_one;
mod part_two;
//use part_one::part_one;
use part_two::part_two;

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
