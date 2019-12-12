use std::fs::File;
use std::io::{Read, Result};

const OPCODE_HALT: u32 = 99;
const OPCODE_ADD: u32 = 1;
const OPCODE_MULTIPLY: u32 = 2;

fn main() -> Result<()> {
    let program = read_program("input")?;
    challenge_1(&program);
    challenge_2(&program);
    Ok(())
}

fn challenge_1(program: &[u32]) {
    let mut program = program.to_owned();
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    println!("Challenge 1: Value left at position 0 = {}", program[0]);
}

fn challenge_2(program: &[u32]) {
    let target = 19_690_720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.to_owned();
            program[1] = noun;
            program[2] = verb;
            run(&mut program);
            if program[0] == target {
                println!("Challenge 2: 100 * noun + verb = {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn read_program(filename: &str) -> Result<Vec<u32>> {
    let mut input: String = String::new();
    File::open(filename)?.read_to_string(&mut input)?;
    let program: Vec<u32> = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();
    Ok(program)
}

fn run(program: &mut Vec<u32>) {
    for i in (0..program.len()).step_by(4) {
        let opcode = program[i];
        if opcode == OPCODE_HALT {
            break;
        }
        let param1 = program[program[i + 1] as usize];
        let param2 = program[program[i + 2] as usize];
        let output_address = program[i + 3] as usize;
        program[output_address] = match opcode {
            OPCODE_ADD => param1 + param2,
            OPCODE_MULTIPLY => param1 * param2,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_addition() {
        let mut arr: Vec<u32> = vec![1, 0, 0, 0, 99];
        run(&mut arr);
        assert_eq!(arr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn performs_multiplication() {
        let mut arr: Vec<u32> = vec![2, 3, 0, 3, 99];
        run(&mut arr);
        assert_eq!(arr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn handles_more_complicated_cases() {
        let mut arr: Vec<u32> = vec![2, 4, 4, 5, 99, 0];
        run(&mut arr);
        assert_eq!(arr, vec![2, 4, 4, 5, 99, 9801]);

        let mut arr: Vec<u32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut arr);
        assert_eq!(arr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn reads_program_from_file() {
        let program = read_program("input").unwrap();
        assert_eq!(program[..5], [1, 0, 0, 3, 1]);
    }
}
