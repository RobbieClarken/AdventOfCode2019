#![allow(dead_code)]
use std::iter;

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input: Signal = std::fs::read_to_string("input").unwrap().trim().into();
    let output = input.fft(100);
    println!(
        "Challenge 1: The first 8 digits of the output are {}",
        &output.to_string()[..8]
    );
}

#[derive(Clone, Debug, PartialEq)]
struct Signal(Vec<i32>);

impl Signal {
    fn fft(&self, phases: usize) -> Self {
        let mut signal = self.clone();
        for _ in 0..phases {
            let mut out: Vec<i32> = Vec::new();
            for i in 0..signal.0.len() {
                out.push(signal.fft_for_digit(i));
            }
            signal = Self(out);
        }
        signal
    }

    fn fft_for_digit(&self, digit: usize) -> i32 {
        let repeats = digit + 1;
        let mut pattern = iter::repeat(PATTERN[0])
            .take(repeats)
            .chain(iter::repeat(PATTERN[1]).take(repeats))
            .chain(iter::repeat(PATTERN[2]).take(repeats))
            .chain(iter::repeat(PATTERN[3]).take(repeats))
            .cycle();
        pattern.next();
        let mut total = 0;
        for (el, multipier) in self.0.iter().zip(pattern) {
            total += el * multipier;
        }
        (total % 10).abs()
    }
}

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_string().parse().unwrap()).collect())
    }
}

impl ToString for Signal {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod test_day_16 {
    use super::*;

    #[test]
    fn validate_small_example() {
        let input: Signal = "12345678".into();
        assert_eq!(input.fft(1), "48226158".into());
        assert_eq!(input.fft(2), "34040438".into());
        assert_eq!(input.fft(3), "03415518".into());
        assert_eq!(input.fft(4), "01029498".into());
    }

    #[test]
    fn validate_large_examples() {
        let input: Signal = "80871224585914546619083218645595".into();
        assert_eq!(&input.fft(100).to_string()[..8], "24176176");

        let input: Signal = "19617804207202209144916044189917".into();
        assert_eq!(&input.fft(100).to_string()[..8], "73745418");

        let input: Signal = "69317163492948606335995924319873".into();
        assert_eq!(&input.fft(100).to_string()[..8], "52432133");
    }
}
