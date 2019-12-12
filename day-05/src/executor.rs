use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::VecDeque;

#[derive(FromPrimitive)]
enum Opcode {
    Halt = 99,
    Add = 1,
    Multiply = 2,
    Input = 3,
}

pub struct Executor<'a> {
    program: &'a mut Vec<i32>,
    input: VecDeque<i32>,
    pos: usize,
}

impl<'a> Executor<'a> {
    pub fn run(program: &'a mut Vec<i32>, input: Vec<i32>) {
        let mut executor = Self {
            program,
            input: VecDeque::from(input),
            pos: 0,
        };
        loop {
            match executor.read_opcode() {
                Opcode::Halt => break,
                Opcode::Input => executor.op_input(),
                Opcode::Add => executor.op_add(),
                Opcode::Multiply => executor.op_multiply(),
            }
        }
    }

    fn read(&mut self) -> i32 {
        let val = self.program[self.pos];
        self.pos += 1;
        val
    }

    fn read_opcode(&mut self) -> Opcode {
        let opcode = self.read();
        FromPrimitive::from_i32(opcode).expect(&format!("unexpected opcode: {}", opcode))
    }

    fn read_param(&mut self) -> i32 {
        let addr = self.read() as usize;
        self.program[addr]
    }

    fn op_input(&mut self) {
        let out_addr = self.read() as usize;
        self.program[out_addr] = self.input.pop_front().expect("insufficient input values");
    }

    fn op_add(&mut self) {
        self.perform_binary_op(|x, y| x + y);
    }

    fn op_multiply(&mut self) {
        self.perform_binary_op(|x, y| x * y);
    }

    fn perform_binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let param1 = self.read_param();
        let param2 = self.read_param();
        let output_address = self.read() as usize;
        self.program[output_address] = op(param1, param2);
    }
}

#[cfg(test)]
mod executor_tests {
    use super::*;

    #[test]
    fn performs_addition() {
        let mut arr: Vec<i32> = vec![1, 0, 0, 0, 99];
        Executor::run(&mut arr, vec![]);
        assert_eq!(arr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn performs_multiplication() {
        let mut arr: Vec<i32> = vec![2, 3, 0, 3, 99];
        Executor::run(&mut arr, vec![]);
        assert_eq!(arr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn handles_input_opcode() {
        let mut arr: Vec<i32> = vec![3, 0, 99];
        Executor::run(&mut arr, vec![101]);
        assert_eq!(arr, vec![101, 0, 99]);

        let mut arr: Vec<i32> = vec![3, 0, 3, 1, 99];
        Executor::run(&mut arr, vec![101, 102]);
        assert_eq!(arr, vec![101, 102, 3, 1, 99]);
    }

    #[test]
    fn handles_more_complicated_cases() {
        let mut arr: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
        Executor::run(&mut arr, vec![]);
        assert_eq!(arr, vec![2, 4, 4, 5, 99, 9801]);

        let mut arr: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        Executor::run(&mut arr, vec![]);
        assert_eq!(arr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
