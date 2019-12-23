use intcode_computer::Computer;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut computer = Computer::load_from_file("input");
    let view = generate_view(&mut computer);
    print!("{}", view);
}

fn generate_view(computer: &mut Computer) -> String {
    let (output, complete) = computer.run(vec![]);
    assert!(complete);
    let view: Vec<_> = output.iter().map(|b| *b as u8).collect();
    String::from_utf8(view).unwrap()
}

#[cfg(test)]
mod test_day_17 {
    use super::*;

    #[test]
    fn generates_output() {
        let mut computer = Computer::load_from_file("../input");
        let out = generate_view(&mut computer);
        let lines: Vec<_> = out.lines().collect();
        assert!(lines.len() > 1);
        assert!(lines[0].len() > 1);
    }
}
