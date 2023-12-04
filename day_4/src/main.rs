mod part_one;
mod part_two;
use part_one::part_one;
use part_two::part_two;

fn main() -> std::io::Result<()> {
    //let result_1 = part_one()?;
    //println!("{}", result_1);
    let result_2 = part_two()?;
    println!("{}", result_2);
    Ok(())
}
