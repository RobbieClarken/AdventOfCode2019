use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::VecDeque;

const MEMORY: usize = 10_000;

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
    AdjustRelativeBase = 9,
}

#[derive(FromPrimitive)]
enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

struct ModeGenerator {
    val: i64,
}

impl ModeGenerator {
    fn next(&mut self) -> Mode {
        let mode = FromPrimitive::from_i64(self.val % 10).unwrap();
        self.val /= 10;
        mode
    }
}

pub struct Computer {
    program: Vec<i64>,
    input: VecDeque<i64>,
    output: Vec<i64>,
    pos: usize,
    relative_base: i64,
}

impl Computer {
    pub fn load(mut program: Vec<i64>) -> Self {
        if program.len() < MEMORY {
            program.resize(MEMORY, 0);
        }
        Self {
            program,
            input: VecDeque::new(),
            pos: 0,
            output: Vec::new(),
            relative_base: 0,
        }
    }

    pub fn load_from_file(path: &str) -> Self {
        let program = std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Self::load(program)
    }

    pub fn run(&mut self, input: Vec<i64>) -> (Vec<i64>, bool) {
        self.input = input.into();
        self.output = Vec::new();

        let completed = loop {
            let start_pos = self.pos;
            match self.read_opcode() {
                (Opcode::Halt, _) => {
                    break true;
                }
                (Opcode::Input, mg) => {
                    if !self.op_input(mg) {
                        self.pos = start_pos;
                        break false;
                    }
                }
                (Opcode::Output, mg) => self.op_output(mg),
                (Opcode::AdjustRelativeBase, mg) => self.op_adjust_relative_base(mg),
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

    fn read(&mut self) -> i64 {
        let val = self.value_at(self.pos);
        self.pos += 1;
        val
    }

    fn value_at(&self, addr: usize) -> i64 {
        *self.program.get(addr).unwrap_or(&0)
    }

    fn read_opcode(&mut self) -> (Opcode, ModeGenerator) {
        let val = self.read();
        let opcode = val % 100;
        let mode_gen = ModeGenerator { val: val / 100 };
        let opcode = FromPrimitive::from_i64(opcode)
            .unwrap_or_else(|| panic!("unexpected opcode: {}", opcode));
        (opcode, mode_gen)
    }

    fn read_param(&mut self, mode_gen: &mut ModeGenerator) -> i64 {
        let v = self.read();
        match mode_gen.next() {
            Mode::Position => self.value_at(v as usize),
            Mode::Immediate => v,
            Mode::Relative => self.value_at((self.relative_base + v) as usize),
        }
    }

    fn read_dest_addr(&mut self, mode_gen: &mut ModeGenerator) -> usize {
        let v = self.read();
        match mode_gen.next() {
            Mode::Position => v as usize,
            Mode::Relative => (self.relative_base + v) as usize,
            Mode::Immediate => unreachable!(),
        }
    }

    fn op_input(&mut self, mut mode_gen: ModeGenerator) -> bool {
        if self.input.is_empty() {
            return false;
        }
        let dest_addr = self.read_dest_addr(&mut mode_gen);
        self.program[dest_addr] = self.input.pop_front().unwrap();
        true
    }

    fn op_output(&mut self, mut mode_gen: ModeGenerator) {
        let v = self.read_param(&mut mode_gen);
        self.output.push(v)
    }

    fn op_adjust_relative_base(&mut self, mut mode_gen: ModeGenerator) {
        self.relative_base += self.read_param(&mut mode_gen);
    }

    fn op_jump_if_true(&mut self, mode_gen: ModeGenerator) {
        self.perform_jump_if(|v| v != 0, mode_gen);
    }

    fn op_jump_if_false(&mut self, mode_gen: ModeGenerator) {
        self.perform_jump_if(|v| v == 0, mode_gen);
    }

    fn perform_jump_if<F>(&mut self, test: F, mut mode_gen: ModeGenerator)
    where
        F: FnOnce(i64) -> bool,
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
        F: FnOnce(i64, i64) -> bool,
    {
        let v1 = self.read_param(&mut mode_gen);
        let v2 = self.read_param(&mut mode_gen);
        let dest_addr = self.read_dest_addr(&mut mode_gen);
        self.program[dest_addr] = if test(v1, v2) { 1 } else { 0 };
    }

    fn op_add(&mut self, mut mode_gen: ModeGenerator) {
        self.perform_binary_op(|x, y| x + y, &mut mode_gen);
    }

    fn op_multiply(&mut self, mut mode_gen: ModeGenerator) {
        self.perform_binary_op(|x, y| x * y, &mut mode_gen);
    }

    fn perform_binary_op<F>(&mut self, op: F, mode_gen: &mut ModeGenerator)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let param1 = self.read_param(mode_gen);
        let param2 = self.read_param(mode_gen);
        let dest_addr = self.read_dest_addr(mode_gen);
        self.program[dest_addr] = op(param1, param2);
    }
}

#[cfg(test)]
mod computer_tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn loads_from_file() {
        let mut path = temp_dir();
        path.push("intcode_computer_loads_from_file.txt");
        let mut file = File::create(&path).unwrap();
        file.write_all(b"104,123,99\n").unwrap();
        let path = path.to_str().unwrap();
        let (out, complete) = Computer::load_from_file(path).run(vec![]);
        assert_eq!(out, vec![123]);
        assert_eq!(complete, true);
    }

    #[test]
    fn performs_addition() {
        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
            3,  // 0: input
            0,  // 1: addr 0
            4,  // 2: output
            0,  // 3: addr 0
            99, // 4: halt
        ];
        let (out, _) = Computer::load(program).run(vec![101]);
        assert_eq!(out, vec![101]);

        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
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

        let program: Vec<i64> = vec![
            11_05, // 0: jump if true
            1,     // 1: true
            5,     // 2: addr 5
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
        let program: Vec<i64> = vec![
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

        let program: Vec<i64> = vec![
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
        let program: Vec<i64> = vec![
            11_07, // 0: less-than
            101,   // 1: 101
            102,   // 2: 102
            0,     // 3: addr 0
            4,     // 4: output
            0,     // 5: addr 0 (value = 1)
            99,    // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![1]);

        let program: Vec<i64> = vec![
            11_07, // 0: less-than
            102,   // 1: 102
            101,   // 2: 101
            0,     // 3: addr 0
            4,     // 4: output
            0,     // 5: addr 0 (value = 0)
            99,    // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);

        let program: Vec<i64> = vec![
            11_07, // 0: less-than
            101,   // 1: 101
            101,   // 2: 101
            0,     // 3: addr 0
            4,     // 4: output
            0,     // 5: addr 0 (value = 0)
            99,    // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);
    }

    #[test]
    fn handles_equals() {
        let program: Vec<i64> = vec![
            11_08, // 0: equals
            101,   // 1: 101
            101,   // 2: 101
            0,     // 3: addr 0
            4,     // 4: output
            0,     // 5: addr 0 (value = 1)
            99,    // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![1]);

        let program: Vec<i64> = vec![
            11_08, // 0: equals
            101,   // 1: 101
            102,   // 2: 102
            0,     // 3: addr 0
            4,     // 4: output
            0,     // 5: addr 0 (value = 0)
            99,    // 6: halt
        ];
        let (out, _) = Computer::load(program).run(vec![]);
        assert_eq!(out, vec![0]);
    }

    #[test]
    fn freezes_execution_if_insufficient_input() {
        let program: Vec<i64> = vec![
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
    fn handles_relative_mode() {
        let program: Vec<i64> = vec![
            204, // 0: output using relative mode
            3,   // 1: relative base + 3 = addr 3 = 101
            99,  // 2: halt
            101, // 3
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![101]);

        let program: Vec<i64> = vec![
            109, // 0: adjust relative base
            8,   // 1: ... to 0 + 8 = 8
            204, // 2: output using relative base
            1,   // 3: relative base + 1 = addr 9 = 111
            109, // 4: adjust relative base
            3,   // 5: ... to 8 + 3 = 11
            204, // 6: output using relative base
            -1,  // 7: relative base - 1 = addr 10 = 222
            99,  // 8: halt
            111, // 9
            222, // 10
            333, // 11
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![111, 222]);
    }

    #[test]
    fn handles_relative_mode_for_input_opcode() {
        let program: Vec<i64> = vec![
            109, // 0: adjust relative base
            6,   // 1: ... to 0 + 6 = 6
            203, // 2: input using relative mode
            1,   // 3: relative base + 1 = @7
            4,   // 4: ouput
            7,   // 5: @7
            99,  // 6: halt
            0,   // 7
        ];
        assert_eq!(Computer::load(program).run(vec![123]).0, vec![123]);
    }

    #[test]
    fn handles_relative_mode_for_binary_operation_output_address() {
        let program: Vec<i64> = vec![
            109,   // 0: adjust relative base
            8,     // 1: ... to 0 + 8 = 8
            21101, // 2: add (immediate, immediate, relative)
            2,     // 3: 2
            3,     // 4: 3
            1,     // 5: relative base @8 + 1 = @9
            4,     // 6: ouput
            9,     // 7: @9
            99,    // 8: halt
            0,     // 9
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![5]);
    }

    #[test]
    fn handles_relative_mode_for_comparison_opcodes() {
        let program: Vec<i64> = vec![
            109,    // 0: adjust relative base
            8,      // 1: ... to 0 + 8 = 8
            211_07, // 2: less-than
            101,    // 3: 101
            102,    // 4: 102
            1,      // 5: relative base @8 + 1 = @9
            4,      // 6: ouput
            9,      // 7: @9
            99,     // 8: halt
            0,      // 9
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![1]);
    }

    #[test]
    fn allows_outputting_beyond_end_of_program() {
        let program: Vec<i64> = vec![
            4,  // 0: output using position mode
            3,  // 1: ... from address 3
            99, // 2: halt
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![0]);

        let program: Vec<i64> = vec![
            204, // 0: output using position mode
            3,   // 1: ... from address 3
            99,  // 2: halt
        ];
        assert_eq!(Computer::load(program).run(vec![]).0, vec![0]);
    }

    #[test]
    fn allows_reading_beyond_program() {
        // This program should run as follows:
        // [@0: JumpIfTrue] jump to @5 if @4 != 0. @4 = 1, therefore jump
        // [@5: Input] input value (0) to @4
        // [@7: JumpIfTrue] jump to location given by @9.
        //                  @9 is beyond the program so should return 0
        //                  *this is what we are testing*
        // [@0: JumpIfTrue] jump to @5 if @4 != 0. @4 = 0, therefore don't jump
        // [@3: Halt] halt
        let program: Vec<i64> = vec![
            10_05, // 0: jump if true
            4,     // 1: @4 = 1 ther 1st time, 0 the 2nd time
            5,     // 2: @5
            99,    // 3: halt
            1,     // 4: place-holder
            3,     // 5: input
            4,     // 6: @4
            11_05, // 7: jump if true (immediate mode)
            1,     // 8: true
        ];
        assert_eq!(Computer::load(program).run(vec![0]).0, vec![]);
    }

    #[test]
    fn can_store_values_beyond_end_of_program() {
        let program: Vec<i64> = vec![
            3,  // 0: input
            5,  // 1: ... to @5
            4,  // 2: output
            5,  // 3: ... from @5
            99, // 4: halt
        ];
        assert_eq!(Computer::load(program).run(vec![101]).0, vec![101]);
    }

    #[test]
    fn can_handle_large_numebrs() {
        let program: Vec<i64> = vec![
            104,              // 0: output
            1125899906842624, // 1: value
            99,               // 2: halt
        ];
        assert_eq!(
            Computer::load(program).run(vec![]).0,
            vec![1125899906842624]
        );
    }

    #[test]
    fn examples() {
        let program: Vec<i64> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![0]);

        let program: Vec<i64> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![0]);

        let program: Vec<i64> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![0]);

        let program: Vec<i64> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![1]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![0]);

        let program: Vec<i64> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(Computer::load(program.clone()).run(vec![0]).0, vec![0]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![1]);

        let program: Vec<i64> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(Computer::load(program.clone()).run(vec![0]).0, vec![0]);
        assert_eq!(Computer::load(program.clone()).run(vec![101]).0, vec![1]);

        let program: Vec<i64> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(Computer::load(program.clone()).run(vec![7]).0, vec![999]);
        assert_eq!(Computer::load(program.clone()).run(vec![8]).0, vec![1000]);
        assert_eq!(Computer::load(program.clone()).run(vec![9]).0, vec![1001]);

        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(
            Computer::load(program.clone()).run(vec![]).0,
            program.clone()
        );

        let out = Computer::load(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0])
            .run(vec![])
            .0;
        assert_eq!(out[0], 1_219_070_632_396_864);
    }
}
