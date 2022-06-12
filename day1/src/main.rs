use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers = BufReader::new(File::open("input1.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        });

    println!(
        "{}",
        numbers
            .into_iter()
            .fold(0, |acc, mass| acc + calc_fuel(mass))
    );
}

fn calc_fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + calc_fuel(fuel)
}
