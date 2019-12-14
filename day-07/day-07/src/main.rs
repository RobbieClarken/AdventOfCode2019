use intcode_computer::Computer;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

const NUM_AMPLIFIERS: u32 = 5;
const NUM_PHASE_OPTIONS: i32 = 5;

fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    let input = load_input("input");
    println!("Challenge 1: Max signal = {}", max_signal(input));
}

fn challenge_2() {
    let input = load_input("input");
    println!(
        "Challenge 2: Max signal = {}",
        max_signal_with_feedback(input)
    );
}

fn load_input(filename: &str) -> Vec<i32> {
    let mut input = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn max_signal(program: Vec<i32>) -> i32 {
    find_max_signal(program, 0, calc_signal)
}

fn max_signal_with_feedback(program: Vec<i32>) -> i32 {
    find_max_signal(program, 5, calc_signal_with_feedback)
}

fn find_max_signal<F>(program: Vec<i32>, phase_offset: i32, signal_func: F) -> i32
where
    F: Fn(Vec<i32>, Vec<i32>) -> i32,
{
    let mut max_signal = i32::min_value();
    for phase_counter in 0..=NUM_PHASE_OPTIONS.pow(NUM_AMPLIFIERS) {
        let phases = phases_from_counter(phase_counter as i32, phase_offset);
        if contains_duplicates(&phases) {
            continue;
        }
        let signal = signal_func(program.clone(), phases);
        if signal > max_signal {
            max_signal = signal;
        }
    }
    max_signal
}

fn phases_from_counter(mut counter: i32, offset: i32) -> Vec<i32> {
    let mut phases = Vec::new();
    for _ in 1..=NUM_AMPLIFIERS {
        phases.push(counter % NUM_PHASE_OPTIONS + offset);
        counter /= NUM_PHASE_OPTIONS;
    }
    phases
}

fn contains_duplicates(values: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for v in values {
        if !seen.insert(v) {
            return true;
        }
    }
    false
}

fn calc_signal(program: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut prev_signal = 0;
    for phase in phases {
        let (out, _) = Computer::load(program.clone()).run(vec![phase, prev_signal]);
        prev_signal = out[0];
    }
    prev_signal
}

fn calc_signal_with_feedback(program: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut computers = Vec::new();
    for _ in 1..=NUM_AMPLIFIERS {
        computers.push(Computer::load(program.clone()));
    }
    for (computer, phase) in computers.iter_mut().zip(&phases) {
        computer.run(vec![*phase]);
    }
    let mut prev_signal = 0;
    loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            let (out, complete) = computer.run(vec![prev_signal]);
            prev_signal = out[0];
            if complete && i == (NUM_AMPLIFIERS - 1) as usize {
                return prev_signal;
            }
        }
    }
}

#[cfg(test)]
mod day_7_tests {
    use super::*;

    #[test]
    fn calculates_signal_for_examples() {
        assert_eq!(
            calc_signal(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,],
                vec![4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            calc_signal(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                vec![0, 1, 2, 3, 4]
            ),
            54321
        );
        assert_eq!(
            calc_signal(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                vec![1, 0, 4, 3, 2]
            ),
            65210
        );
    }

    #[test]
    fn finds_max_signal_for_examples() {
        assert_eq!(
            max_signal(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
        assert_eq!(
            max_signal(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],),
            54321
        );
        assert_eq!(
            max_signal(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],),
            65210
        );
    }

    #[test]
    fn calculates_signal_with_feedback_for_examples() {
        assert_eq!(
            calc_signal_with_feedback(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                vec![9, 8, 7, 6, 5]
            ),
            139629729
        );
        assert_eq!(
            calc_signal_with_feedback(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                vec![9, 7, 8, 5, 6]
            ),
            18216
        );
    }

    #[test]
    fn finds_max_signal_with_feedback_for_examples() {
        assert_eq!(
            max_signal_with_feedback(vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ],),
            139629729
        );
        assert_eq!(
            max_signal_with_feedback(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ],),
            18216
        );
    }
}
