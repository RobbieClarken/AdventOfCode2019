use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::VecDeque;

#[derive(FromPrimitive, Debug)]
enum Opcode {
    Halt = 99,
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
}

#[derive(FromPrimitive)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

struct ModeGenerator {
    val: i32,
}

impl ModeGenerator {
    fn next(&mut self) -> Mode {
        let mode = FromPrimitive::from_i32(self.val % 10).unwrap();
        self.val /= 10;
        mode
    }
}

pub struct Computer {
    program: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
    pos: usize,
}

impl Computer {
    pub fn load(program: Vec<i32>) -> Self {
        Self {
            program,
            input: VecDeque::new(),
            pos: 0,
            output: Vec::new(),
        }
    }

    pub fn run(&mut self, input: Vec<i32>) -> (Vec<i32>, bool) {
        self.input = input.into();
        self.output = Vec::new();

        let completed = loop {
            let start_pos = self.pos;
            match self.read_opcode() {
                (Opcode::Halt, _) => {
                    break true;
                }
                (Opcode::Input, _) => {
                    if !self.op_input() {
                        self.pos = start_pos;
                        break false;
                    }
                }
                (Opcode::Output, mg) => self.op_output(mg),
                (Opcode::Add, mg) => self.op_add(mg),
                (Opcode::Multiply, mg) => self.op_multiply(mg),
                (Opcode::JumpIfTrue, mg) => self.op_jump_if_true(mg),
                (Opcode::JumpIfFalse, mg) => self.op_jump_if_false(mg),
                (Opcode::LessThan, mg) => self.op_less_than(mg),
                (Opcode::Equals, mg) => self.op_equals(mg),
            }
        };
        (self.output.clone(), completed)
    }

    fn read(&mut self) -> i32 {
        let val = self.program[self.pos];
        self.pos += 1;
        val
    }

    fn read_opcode(&mut self) -> (Opcode, ModeGenerator) {
        let val = self.read();
        let opcode = val % 100;
        let mode_gen = ModeGenerator { val: val / 100 };
        let opcode = FromPrimitive::from_i32(opcode)
            .unwrap_or_else(|| panic!("unexpected opcode: {}", opcode));
        (opcode, mode_gen)
    }

    fn read_param(&mut self, mode_gen: &mut ModeGenerator) -> i32 {
        let mut v = self.read();
        if let Mode::Position = mode_gen.next() {
            v = self.program[v as usize];
        }
        v
    }

    fn op_input(&mut self) -> bool {
        if self.input.is_empty() {
            return false;
        }
        let out_addr = self.read() as usize;
        self.program[out_addr] = self.input.pop_front().expect("insufficient input values");
        true
    }

    fn op_output(&mut self, mut mode_gen: ModeGenerator) {
        let v = self.read_param(&mut mode_gen);
        self.output.push(v)
    }

    fn op_jump_if_true(&mut self, mode_gen: ModeGenerator) {
        self.perform_jump_if(|v| v != 0, mode_gen);
    }

    fn op_jump_if_false(&mut self, mode_gen: ModeGenerator) {
        self.perform_jump_if(|v| v == 0, mode_gen);
    }

    fn perform_jump_if<F>(&mut self, test: F, mut mode_gen: ModeGenerator)
    where
        F: FnOnce(i32) -> bool,
    {
        let test_val = self.read_param(&mut mode_gen);
        let dest = self.read_param(&mut mode_gen) as usize;
        if test(test_val) {
            self.pos = dest;
        }
    }

    fn op_less_than(&mut self, mode_gen: ModeGenerator) {
        self.perform_comparison(|v1, v2| v1 < v2, mode_gen);
    }

    fn op_equals(&mut self, mode_gen: ModeGenerator) {
        self.perform_comparison(|v1, v2| v1 == v2, mode_gen);
    }

    fn perform_comparison<F>(&mut self, test: F, mut mode_gen: ModeGenerator)
    where
        F: FnOnce(i32, i32) -> bool,
    {
        let v1 = self.read_param(&mut mode_gen);
        let v2 = self.read_param(&mut mode_gen);
        let dest = self.read() as usize;
        self.program[dest] = if test(v1, v2) { 1 } else { 0 };
    }

    fn op_add(&mut self, mut mode_gen: ModeGenerator) {
        self.perform_binary_op(|x, y| x + y, &mut mode_gen);
    }

    fn op_multiply(&mut self, mut mode_gen: ModeGenerator) {
        self.perform_binary_op(|x, y| x * y, &mut mode_gen);
    }

    fn perform_binary_op<F>(&mut self, op: F, mode_gen: &mut ModeGenerator)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let param1 = self.read_param(mode_gen);
        let param2 = self.read_param(mode_gen);
        let output_address = self.read() as usize;
        self.program[output_address] = op(param1, param2);
    }
}

#[cfg(test)]
mod computer_tests {
    use super::*;

    #[test]
    fn performs_addition() {
        let program: Vec<i32> = vec![
            1,  // 0: add
            7,  // 1: addr 7 = 2
            8,  // 2: addr 8 = 3,
            0,  // 3: addr 0
            4,  // 4: output
            0,  // 5: addr 0
            99, // 6: halt
            2,  // 7
            3,  // 8
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![5]);
    }

    #[test]
    fn performs_multiplication() {
        let program: Vec<i32> = vec![
            2,  // 0: multiply
            7,  // 1: addr 7 = 2
            8,  // 2: addr 8 = 2,
            0,  // 3: addr 0
            4,  // 4: output
            0,  // 5: addr 0
            99, // 6: halt
            2,  // 7
            3,  // 8
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![6]);
    }

    #[test]
    fn handles_input_opcode() {
        let program: Vec<i32> = vec![
            3,  // 0: input
            0,  // 1: addr 0
            4,  // 2: output
            0,  // 3: addr 0
            99, // 4: halt
        ];
        let (out, _) = Computer::load(program).run(vec![101]);
        assert_eq!(out, vec![101]);

        let program: Vec<i32> = vec![
            3,  // 0: input
            0,  // 1: addr 0
            3,  // 2: input
            1,  // 3: addr 1
            4,  // 4: output
            1,  // 5: addr 1
            99, // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![101, 102]);
        assert_eq!(out, vec![102]);
    }

    #[test]
    fn handles_output_opcode() {
        let program = vec![
            4,  // 0: output
            0,  // 1: addr 0 = 4
            4,  // 2: output
            4,  // 3: addr 4 = 99
            99, // 4: halt
        ];
        let (out, _) = Computer::load(program).run(vec![101]);
        assert_eq!(out, vec![4, 99]);
    }

    #[test]
    fn handles_immediate_mode_for_first_param() {
        let program: Vec<i32> = vec![
            101, // 0: add (param 1 is immediate mode)
            20,  // 1: immediate mode value = 20
            7,   // 2: addr 7 value = 30
            0,   // 3: write to addr 0
            4,   // 4: output
            0,   // 5: value in addr 0
            99,  // 6: halt
            30,  // 7
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![50]);
    }

    #[test]
    fn handles_immediate_mode_for_second_param() {
        let program: Vec<i32> = vec![
            1001, // 0: add (param 2 is immediate mode)
            7,    // 1: addr 7 value = 20
            30,   // 2: immediate mode value value = 30
            0,    // 3: write to addr 0
            4,    // 4: output
            0,    // 5: value in addr 0
            99,   // 6: halt
            20,   // 7
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![50]);
    }

    #[test]
    fn handles_jump_if_true() {
        let program: Vec<i32> = vec![
            11_05, // 0: jump if true
            0,     // 1: false
            5,     // 2: addr 5 (not used)
            1_04,  // 3: output
            101,   // 4: value 101
            1_04,  // 5: output
            102,   // 6: value 102
            99,    // 7: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![101, 102]);

        let program: Vec<i32> = vec![
            11_05, // 0: jump if true
            1,     // 1: true
            5,     // 2: addr 5 (jumped)
            1_04,  // 3: output (jumped)
            101,   // 4: value 101
            1_04,  // 5: output
            102,   // 6: value 102
            99,    // 7: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![102]);
    }

    #[test]
    fn handles_jump_if_false() {
        let program: Vec<i32> = vec![
            11_06, // 0: jump if false
            0,     // 1: false
            5,     // 2: addr 5
            1_04,  // 3: output (jumped)
            101,   // 4: value 101 (jumped)
            1_04,  // 5: output
            102,   // 6: value 102
            99,    // 7: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![102]);

        let program: Vec<i32> = vec![
            11_06, // 0: jump if true
            1,     // 1: true
            5,     // 2: addr 5 (not used)
            1_04,  // 3: output
            101,   // 4: value 101
            1_04,  // 5: output
            102,   // 6: value 102
            99,    // 7: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![101, 102]);
    }

    #[test]
    fn handles_less_than() {
        let program: Vec<i32> = vec![
            111_07, // 0: less-than
            101,    // 1: 101
            102,    // 2: 102
            0,      // 3: addr 0
            4,      // 4: output
            0,      // 5: addr 0 (value = 1)
            99,     // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![1]);

        let program: Vec<i32> = vec![
            111_07, // 0: less-than
            102,    // 1: 102
            101,    // 2: 101
            0,      // 3: addr 0
            4,      // 4: output
            0,      // 5: addr 0 (value = 0)
            99,     // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);

        let program: Vec<i32> = vec![
            111_07, // 0: less-than
            101,    // 1: 101
            101,    // 2: 101
            0,      // 3: addr 0
            4,      // 4: output
            0,      // 5: addr 0 (value = 0)
            99,     // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);
    }

    #[test]
    fn handles_equals() {
        let program: Vec<i32> = vec![
            111_08, // 0: equals
            101,    // 1: 101
            101,    // 2: 101
            0,      // 3: addr 0
            4,      // 4: output
            0,      // 5: addr 0 (value = 1)
            99,     // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![1]);

        let program: Vec<i32> = vec![
            111_08, // 0: equals
            101,    // 1: 101
            102,    // 2: 102
            0,      // 3: addr 0
            4,      // 4: output
            0,      // 5: addr 0 (value = 0)
            99,     // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);
    }

    #[test]
    fn freezes_execution_if_insufficient_input() {
        let program: Vec<i32> = vec![
            3,  // 0: input
            0,  // 1: addr 0
            4,  // 2: output
            8,  // 3: addr 8 = 99
            3,  // 4: input
            1,  // 5: addr 1
            4,  // 6: output
            1,  // 7: addr 1 = 102
            99, // 8: halt
        ];

        let mut computer = Computer::load(program);
        let (out, complete) = computer.run(vec![101]);
        assert_eq!(out, vec![99]);
        assert_eq!(complete, false);

        let (out, complete) = computer.run(vec![102]);
        assert_eq!(out, vec![102]);
        assert_eq!(complete, true);
    }

    #[test]
    fn examples() {
        let program: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![0]);

        let program: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![0]);

        let program: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![0]);

        let program: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![0]);

        let program: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(Computer::load(program.clone()).run(vec![0]).0, vec![0]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![1]);

        let program: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(Computer::load(program.clone()).run(vec![0]).0, vec![0]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![1]);

        let program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![999]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1000]);
        assert_eq!(Computer::load(program.clone()).run(vec![9]).0, vec![1001]);
    }
}
