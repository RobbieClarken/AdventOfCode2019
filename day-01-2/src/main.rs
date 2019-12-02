use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_required(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + fuel_required(fuel)
    } else {
        0
    }
}

fn main() -> std::io::Result<()> {
    let out: i32 = BufReader::new(File::open("input")?)
        .lines()
        .map(|v| fuel_required(v.unwrap().parse().unwrap()))
        .sum();
    println!("{}", out);
    Ok(())
}
