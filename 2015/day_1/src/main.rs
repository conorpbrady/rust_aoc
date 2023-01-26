use aoc_input::aoc_input;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = aoc_input(args);

    let mut floor: i32 = 0;
    let mut basement: bool = false;

    let input = match input {
        Ok(v) => v,
        Err(e) =>  { panic!("Could not read input: {}", e); },
    };

    for (i, c) in input[0].chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1  && !basement {
            println!("Entered basement on floor {}", i+1);
            basement = true;
        }
    }
    println!("Lands on Floor {}", floor);

}
