use intcode_computer::Computer;

fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    println!("Challenge 1: BOOST keycode = {}", run(1));
}

fn challenge_2() {
    println!("Challenge 2: Coordinates = {}", run(2));
}

fn run(input: i64) -> i64 {
    let (out, complete) = Computer::load_from_file("input").run(vec![input]);
    assert!(complete);
    assert_eq!(out.len(), 1);
    out[0]
}
