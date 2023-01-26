use aoc_input::aoc_input;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = aoc_input(args).expect("Couldn't get input");

    let mut area_sum: i32 = 0;
    let mut ribbon_sum: i32 = 0;

    for line in input {

        let dims: Vec<String> = line.split("x").map(|s| s.to_string()).collect();
        let w: i32 = dims[0].parse().unwrap();
        let h: i32 = dims[1].parse().unwrap();
        let l: i32 = dims[2].parse().unwrap();

        let a = w*h;
        let b = w*l;
        let c = l*h;
        let scrap = [a, b, c].iter().copied().min().unwrap();

        let area = 2*a + 2*b + 2*c + scrap;
        area_sum += area;
        // println!("{} | {} | {} || {}", w, h, l, area);

        let d = 2 * (w + h);
        let e = 2 * (w + l);
        let f = 2 * (l + h);

        let ribbon = [d, e, f].iter().copied().min().unwrap() + w*h*l;
        ribbon_sum += ribbon
    }

    println!("Total sqft: {} | Total ribbon: {}", area_sum, ribbon_sum);
}
