use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let out: i32 = BufReader::new(File::open("input")?)
        .lines()
        .map(|v| {
            let v: i32 = v.unwrap().parse().unwrap();
            v / 3 - 2
        })
        .sum();
    println!("{}", out);
    Ok(())
}
