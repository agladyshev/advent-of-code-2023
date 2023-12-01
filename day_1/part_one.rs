use std::fs::File;
use std::io::Read;

fn is_num(byte: u8) -> bool {
    if byte >= b'0' && byte <= b'9' {
        true
    } else {
        false
    }
}

fn ascii_to_int(byte: u8) -> Option<u8> {
    if is_num(byte) {
        Some(byte - b'0')
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut total: u16 = 0;
    let mut file = File::open("input")?;
    let mut num_1 = None;
    let mut num_2 = None;
    loop {
        let read_count = file.read(&mut buffer)?;
        for &byte in buffer.iter().take(read_count) {
            if is_num(byte) {
                if num_1.is_none() {
                    num_1 = Some(byte);
                }
                num_2 = Some(byte);
            } else if byte == 10 {
                total += (ascii_to_int(num_1.expect("num_1 is empty"))
                    .expect("num_1 is not a valid digit")
                    * 10) as u16;
                total += (ascii_to_int(num_2.expect("num_2 is empty"))
                    .expect("num_2 is not a valid digit")) as u16;
                num_1 = None;
                num_2 = None;
            }
        }
        if read_count != BUFFER_LEN {
            break;
        }
    }
    println!("{}", total);
    Ok(())
}
