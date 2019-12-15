use std::fs::File;
use std::io::Read;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let solution = solve_challenge_1(read_data(), 25, 6);
    println!("Challenge 1: Solution = {}", solution);
}

fn solve_challenge_1(data: Vec<u32>, width: usize, height: usize) -> u32 {
    let data = build_layers(data, width, height);
    let idx = layer_with_least_of_digit(0, &data);
    let layer = &data[idx];
    count_digit(1, layer) * count_digit(2, layer)
}

fn read_data() -> Vec<u32> {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect()
}

fn build_layers(data: Vec<u32>, width: usize, height: usize) -> Vec<Vec<u32>> {
    let mut layers = Vec::new();
    let num_layers = data.len() / (width * height);
    for layer_index in 0..num_layers {
        let start = layer_index * width * height;
        let end = start + width * height;
        layers.push(data[start..end].to_vec());
    }
    layers
}

fn layer_with_least_of_digit(digit: u32, data: &[Vec<u32>]) -> usize {
    let mut best_layer = 0;
    let mut best_count = u32::max_value();
    for (i, layer) in data.iter().enumerate() {
        let count = count_digit(digit, layer);
        if count < best_count {
            best_layer = i;
            best_count = count;
        }
    }
    best_layer
}

fn count_digit(digit: u32, values: &[u32]) -> u32 {
    let mut count = 0;
    for v in values {
        if *v == digit {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test_day_8 {
    use super::*;

    #[test]
    fn reads_data() {
        let data = read_data();
        assert_eq!(data.len(), 15_000);
        assert_eq!(data[..10], [1, 2, 2, 2, 2, 2, 2, 2, 2, 0]);
    }

    #[test]
    fn solves_challenge_1() {
        let data = vec![
            1, 1, 2, // layer 1,
            2, 2, 2, //
            1, 2, 2, // layer 2
            2, 2, 0, //
        ];
        assert_eq!(solve_challenge_1(data, 3, 2), 8);

        let data = vec![
            0, 0, 1, 1, // layer 1,
            1, 1, 1, 2, //
            1, 1, 1, 2, // layer 2
            2, 2, 2, 0, //
        ];
        assert_eq!(solve_challenge_1(data, 4, 2), 12);
    }
}
