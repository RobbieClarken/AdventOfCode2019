use intcode_computer::Computer;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut computer = Computer::load_from_file("input");
    let program = "
            NOT A T
            OR T J
            NOT B T
            OR T J
            NOT C T
            OR T J
            AND D J
            WALK
        "
    .trim()
    .lines()
    .collect();
    let (output, complete) = computer.run(to_ascii(program));
    assert!(complete);
    let last_out_val = output[output.len() - 1];
    if last_out_val > 127 {
        println!("Challenge 1: Damage to the hull = {}", last_out_val);
    } else {
        for c in output {
            print!("{}", c as u8 as char);
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Instruction {
    AND(char, char),
    OR(char, char),
    NOT(char, char),
    WALK,
}

use Instruction::*;

impl Instruction {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        match parts[0] {
            "AND" => AND(to_char(parts[1]), to_char(parts[2])),
            "OR" => OR(to_char(parts[1]), to_char(parts[2])),
            "NOT" => NOT(to_char(parts[1]), to_char(parts[2])),
            "WALK" => WALK,
            _ => unreachable!(),
        }
    }

    fn to_ascii(&self) -> Vec<i64> {
        let s = match self {
            AND(a, b) => format!("AND {} {}", a, b),
            OR(a, b) => format!("OR {} {}", a, b),
            NOT(a, b) => format!("NOT {} {}", a, b),
            WALK => "WALK".to_owned(),
        };
        s.chars().map(|c| c as i64).collect()
    }
}

fn to_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn to_ascii(program: Vec<&str>) -> Vec<i64> {
    let mut out = vec![];
    for instruction in program {
        out.extend(Instruction::new(instruction).to_ascii());
        out.push('\n' as i64);
    }
    out
}

#[cfg(test)]
mod test_day_21 {
    use super::*;
    use std::collections::HashMap;

    impl Instruction {
        fn execute(&self, readonly_regs: &[bool], reg_t_j: (bool, bool)) -> (bool, bool) {
            let mut regs: HashMap<char, bool> = HashMap::new();
            regs.insert('A', readonly_regs[0]);
            regs.insert('B', readonly_regs[1]);
            regs.insert('C', readonly_regs[2]);
            regs.insert('D', readonly_regs[3]);
            regs.insert('T', reg_t_j.0);
            regs.insert('J', reg_t_j.1);
            match self {
                AND(x, y) => {
                    let x_val = *regs.get(x).unwrap();
                    let y_val = *regs.get(y).unwrap();
                    regs.insert(*y, x_val && y_val);
                }
                OR(x, y) => {
                    let x_val = *regs.get(x).unwrap();
                    let y_val = *regs.get(y).unwrap();
                    regs.insert(*y, x_val || y_val);
                }
                NOT(x, y) => {
                    let x_val = *regs.get(x).unwrap();
                    regs.insert(*y, !x_val);
                }
                WALK => {}
            };
            println!(
                "{:>16} -- A:{} B:{} C:{} D:{} T:{} J:{}",
                format!("{:?}", self),
                *regs.get(&'A').unwrap() as u8,
                *regs.get(&'B').unwrap() as u8,
                *regs.get(&'C').unwrap() as u8,
                *regs.get(&'D').unwrap() as u8,
                *regs.get(&'T').unwrap() as u8,
                *regs.get(&'J').unwrap() as u8,
            );
            (*regs.get(&'T').unwrap(), *regs.get(&'J').unwrap())
        }
    }

    fn test_program(program: &str, platform: &str) -> bool {
        let program: Vec<_> = program
            .trim()
            .lines()
            .map(|i| Instruction::new(i))
            .collect();
        let platform: Vec<bool> = platform.chars().map(|c| c == '#').collect();
        let mut pos = 0;
        while pos < platform.len() - 4 {
            println!("{}", pos);
            if !platform[pos] {
                return false;
            }
            let mut registries = (false, false);
            for instruction in &program {
                registries = instruction.execute(&platform[pos + 1..=pos + 4], registries);
            }
            if registries.1 {
                pos += 4;
            } else {
                pos += 1;
            }
        }
        true
    }

    #[test]
    fn converts_instructions_to_ascii() {
        let program = vec!["NOT D J", "WALK"];
        assert_eq!(
            to_ascii(program),
            vec![78, 79, 84, 32, 68, 32, 74, 10, 87, 65, 76, 75, 10]
        );
    }

    #[test]
    fn trims_spaces() {
        let program = "
            NOT D J
            WALK
        "
        .trim()
        .lines()
        .collect();
        assert_eq!(
            to_ascii(program),
            vec![78, 79, 84, 32, 68, 32, 74, 10, 87, 65, 76, 75, 10]
        );
    }

    #[test]
    fn jumps_known_holes() {
        let program = "
            NOT A T
            OR T J
            NOT B T
            OR T J
            NOT C T
            OR T J
            AND D J
            WALK
        ";
        assert!(test_program(&program, "#################"));
        assert!(test_program(&program, "#####.###########"));
        assert!(test_program(&program, "#####..#.########"));
        assert!(test_program(&program, "#####...#########"));
    }
}
