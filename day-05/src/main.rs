mod executor;

use executor::Executor;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let program = read_program("input")?;
    challenge_1(&program);
    Ok(())
}

fn challenge_1(program: &[i32]) {
    let mut program = program.to_owned();
    Executor::run(&mut program, vec![]);
}

fn read_program(filename: &str) -> Result<Vec<i32>> {
    let mut input: String = String::new();
    File::open(filename)?.read_to_string(&mut input)?;
    let program: Vec<i32> = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    Ok(program)
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn reads_program_from_file() {
        let program = read_program("input").unwrap();
        assert_eq!(program[..5], [3, 225, 1, 225, 6]);
    }
}
