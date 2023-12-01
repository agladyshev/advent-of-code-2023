use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    const BUFFER_LEN:usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut total = 0;
    let mut file = File::open("input")?;
    loop {
        let read_count = file.read(&mut buffer)?;
        println!("{}", read_count);
        if read_count != BUFFER_LEN {
            break;
        }
    }
    println!("{}", total);
    Ok(())
}
