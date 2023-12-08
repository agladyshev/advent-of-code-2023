use std::collections::HashMap;

pub fn part_one(
    actions: &Vec<bool>,
    map: &HashMap<String, (String, String)>,
) -> Result<usize, std::io::Error> {
    let mut steps = 0;
    let mut position = "AAA";
    loop {
        for action in actions {
            let options = map.get(position).expect("Bad map");
            steps += 1;
            let next = if *action { &options.1 } else { &options.0 };
            //println!("{}", next);
            if next == "ZZZ" {
                return Ok(steps);
            } else {
                position = next;
            }
        }
        //println!("loop");
        //return Ok(steps);
        if steps > 99999999999 {
            println!("too many steps");
            return Ok(steps);
        }
    }
}
