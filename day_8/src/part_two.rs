use std::collections::HashMap;

pub fn part_two(
    actions: &Vec<bool>,
    map: &HashMap<String, (String, String)>,
) -> Result<usize, std::io::Error> {
    let mut positions: Vec<&str> = Vec::new();
    for key in map.keys() {
        if key.chars().last().expect("") == 'A' {
            positions.push(key);
        }
    }
    println!("{:?}", positions);
    let mut results: Vec<usize> = Vec::new();
    for position in positions {
        results.push(get_steps(&position, actions, map));
    }
    println!("{:?}", results);
    Ok(lcm(results))
}

fn get_steps(start: &str, actions: &Vec<bool>, map: &HashMap<String, (String, String)>) -> usize {
    let mut steps = 0;
    let mut position = start;
    loop {
        for action in actions {
            let options = map.get(position).expect("Bad map");
            steps += 1;
            let next = if *action { &options.1 } else { &options.0 };
            //println!("{}", next);
            if next.chars().last().expect("") == 'Z' {
                return steps;
            } else {
                position = next;
            }
        }
        if steps > 99999999999 {
            println!("too many steps");
            return steps;
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(numbers: Vec<usize>) -> usize {
    let len = numbers.len();
    let mut res = numbers[0];
    for i in 0..len {
        res = (numbers[i] * res) / gcd(numbers[i], res);
    }
    return res;
}
