mod executor;

use executor::Executor;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let program = read_program("input")?;
    challenge_1(program.clone());
    challenge_2(program);
    Ok(())
}

fn challenge_1(program: Vec<i32>) {
    let output = Executor::run(program, vec![1]);
    for err_code in output[..output.len() - 1].iter() {
        assert_eq!(*err_code, 0);
    }
    println!("Challenge 1: Diagnostic code = {}", output.last().unwrap());
}

fn challenge_2(program: Vec<i32>) {
    let output = Executor::run(program, vec![5]);
    for err_code in output[..output.len() - 1].iter() {
        assert_eq!(*err_code, 0);
    }
    println!("Challenge 2: Diagnostic code = {}", output.last().unwrap());
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
