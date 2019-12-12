use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::{Read, Result};


#[derive(FromPrimitive)]
enum Opcode {
    Halt = 99,
    Add = 1,
    Multiply = 2,
}

fn main() -> Result<()> {
    let program = read_program("input")?;
    challenge_1(&program);
    Ok(())
}

fn challenge_1(program: &[i32]) {
    let mut program = program.to_owned();
    run(&mut program);
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

fn run(program: &mut Vec<i32>) {
    for i in (0..program.len()).step_by(4) {
        let opcode = program[i];
        if opcode == Opcode::Halt as i32 {
            break;
        }
        let param1 = program[program[i + 1] as usize];
        let param2 = program[program[i + 2] as usize];
        let output_address = program[i + 3] as usize;
        program[output_address] = match FromPrimitive::from_i32(opcode) {
            Some(Opcode::Add) => param1 + param2,
            Some(Opcode::Multiply) => param1 * param2,
            _ => panic!("invalid opcode: {}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_addition() {
        let mut arr: Vec<i32> = vec![1, 0, 0, 0, 99];
        run(&mut arr);
        assert_eq!(arr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn performs_multiplication() {
        let mut arr: Vec<i32> = vec![2, 3, 0, 3, 99];
        run(&mut arr);
        assert_eq!(arr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn handles_more_complicated_cases() {
        let mut arr: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
        run(&mut arr);
        assert_eq!(arr, vec![2, 4, 4, 5, 99, 9801]);

        let mut arr: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut arr);
        assert_eq!(arr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn reads_program_from_file() {
        let program = read_program("input").unwrap();
        assert_eq!(program[..5], [3, 225, 1, 225, 6]);
    }
}
