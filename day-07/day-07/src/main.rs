use intcode_computer::Computer;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

const NUM_AMPLIFIERS: u32 = 5;
const NUM_PHASE_OPTIONS: i32 = 5;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let input: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    println!("Challenge 1: Max signal = {}", max_signal(input));
}

fn max_signal(program: Vec<i32>) -> i32 {
    let mut max_signal = i32::min_value();
    for phase_counter in 0..=NUM_PHASE_OPTIONS.pow(NUM_AMPLIFIERS) {
        let phases = phases_from_counter(phase_counter as i32);
        if contains_duplicates(&phases) {
            continue;
        }
        let signal = calc_signal(program.clone(), phases);
        if signal > max_signal {
            max_signal = signal;
        }
    }
    max_signal
}

fn phases_from_counter(mut counter: i32) -> Vec<i32> {
    let mut phases = Vec::new();
    for _ in 1..=NUM_AMPLIFIERS {
        let p = counter % NUM_PHASE_OPTIONS;
        phases.push(p);
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

#[allow(dead_code)]
fn calc_signal(program: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut prev_signal = 0;
    for phase in phases {
        let (out, _) = Computer::load(program.clone()).run(vec![phase, prev_signal]);
        prev_signal = out[0];
    }
    prev_signal
}

#[cfg(test)]
mod day_7_tests {
    use super::*;

    #[test]
    fn calculates_calc_signal_for_examples() {
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
}
