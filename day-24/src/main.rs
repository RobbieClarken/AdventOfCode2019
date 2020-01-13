use std::collections::HashSet;
use std::fmt::Write;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let state = State::new(&input);
    let repeated_state = first_repeated_state(&state);
    println!(
        "Challenge 1: Biodiversity rating of first repeated state = {}",
        repeated_state.biodiversity()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellState {
    Infested,
    Empty,
}

use CellState::*;

impl Into<char> for &CellState {
    fn into(self) -> char {
        match self {
            Infested => '#',
            Empty => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    tiles: Vec<Vec<CellState>>,
    max_x: usize,
    max_y: usize,
}

impl State {
    fn new(s: &str) -> Self {
        let tiles: Vec<Vec<_>> = s
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| if c == '#' { Infested } else { Empty })
                    .collect()
            })
            .collect();
        Self {
            max_x: tiles[0].len() - 1,
            max_y: tiles.len() - 1,
            tiles,
        }
    }

    fn next(&self) -> Self {
        let mut new_tiles: Vec<Vec<_>> = (0..=self.max_y)
            .map(|_| [Empty].repeat(self.max_x + 1))
            .collect();
        for (y, row) in new_tiles.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = self.next_cell_state(x, y);
            }
        }
        Self {
            tiles: new_tiles,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn next_cell_state(&self, x: usize, y: usize) -> CellState {
        let mut neighbours_infested = 0;
        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let x_neighbour = x as isize + dx;
            let y_neighbour = y as isize + dy;
            if x_neighbour < 0
                || y_neighbour < 0
                || x_neighbour > self.max_x as isize
                || y_neighbour > self.max_y as isize
            {
                continue;
            }
            if let Infested = self.tiles[y_neighbour as usize][x_neighbour as usize] {
                neighbours_infested += 1;
            }
        }
        match (self.tiles[y][x], neighbours_infested) {
            (Infested, 1) | (Empty, 1) | (Empty, 2) => Infested,
            _ => Empty,
        }
    }

    fn biodiversity(&self) -> u64 {
        self.tiles
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, cell)| match cell {
                Infested => 2u64.pow(i as u32),
                Empty => 0,
            })
            .sum()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.tiles.iter().for_each(|row| {
            row.iter().for_each(|c| f.write_char(c.into()).unwrap());
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

fn first_repeated_state(state: &State) -> State {
    let mut state = state.clone();
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(state.clone()) {
            break state;
        }
        state = state.next();
    }
}

#[cfg(test)]
mod test_day_24 {
    use super::*;

    #[test]
    fn evolves_state() {
        let state = State::new(
            "
....#
#..#.
#..##
..#..
#....
",
        );

        let expected = "
#..#.
####.
###.#
##.##
.##..
"
        .trim_start();
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "
#####
....#
....#
...#.
#.###
"
        .trim_start();
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "
#....
####.
...##
#.##.
.##.#
"
        .trim_start();
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "
####.
....#
##..#
.....
##...
"
        .trim_start();
        let state = state.next();
        assert_eq!(state.to_string(), expected);
    }

    #[test]
    fn finds_first_repeated_state() {
        let state = State::new(
            "
....#
#..#.
#..##
..#..
#....
",
        );
        let state = first_repeated_state(&state);
        let expected = "
.....
.....
.....
#....
.#...
"
        .trim_start();
        assert_eq!(state.to_string(), expected);
    }

    #[test]
    fn calculates_biodiversity() {
        let state = State::new(
            "
.....
.....
.....
#....
.#...
",
        );
        assert_eq!(state.biodiversity(), 2129920);
    }
}
