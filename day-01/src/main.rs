use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input = read_input("input")?;
    challenge_1(&input);
    challenge_2(&input);
    Ok(())
}

fn challenge_1(input: &[i32]) {
    let output: i32 = input.iter().map(|v| v / 3 - 2).sum();
    println!("Challenge 1: {}", output);
}

fn challenge_2(input: &[i32]) {
    let output: i32 = input.iter().map(|v| fuel_required(*v)).sum();
    println!("Challenge 2: {}", output);
}

fn read_input(filename: &str) -> Result<Vec<i32>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|v| v.unwrap().parse().unwrap())
        .collect())
}

fn fuel_required(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + fuel_required(fuel)
    } else {
        0
    }
}
