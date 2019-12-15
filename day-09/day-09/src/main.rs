use intcode_computer::Computer;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let (out, complete) = Computer::load_from_file("input").run(vec![1]);
    assert!(complete);
    assert_eq!(out.len(), 1);
    println!("Challenge 1: BOOST keycode = {}", out[0]);
}
