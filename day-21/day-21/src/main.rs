use intcode_computer::Computer;

fn main() {
    let mut computer = Computer::load_from_file("input");
    let program = to_ascii(&[
        NOT('A', 'J'),
        NOT('B', 'T'),
        AND('T', 'J'),
        NOT('C', 'T'),
        AND('T', 'J'),
        AND('D', 'J'),
        WALK,
    ]);
    let (output, complete) = computer.run(program);
    assert!(complete);
    for c in output {
        print!("{}", c as u8 as char);
    }
}

#[allow(dead_code)]
enum Instruction {
    AND(char, char),
    OR(char, char),
    NOT(char, char),
    WALK,
}

use Instruction::*;

impl Instruction {
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

fn to_ascii(program: &[Instruction]) -> Vec<i64> {
    let mut out = vec![];
    for instruction in program {
        out.extend(instruction.to_ascii());
        out.push('\n' as i64);
    }
    out
}

#[cfg(test)]
mod test_day_21 {
    use super::*;

    #[test]
    fn converts_instructions_to_ascii() {
        let program = vec![Instruction::NOT('D', 'J'), Instruction::WALK];
        assert_eq!(
            to_ascii(&program),
            vec![78, 79, 84, 32, 68, 32, 74, 10, 87, 65, 76, 75, 10]
        );
    }
}
