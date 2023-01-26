use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn aoc_input(args: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let mut filepath = "input.txt";
    if args.len() > 1 {
        filepath = "test.txt";
    }
    let file = File::open(filepath).expect("No file found");
    let buf = BufReader::new(file);
    let data = buf.lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect();
    Ok(data)
}
