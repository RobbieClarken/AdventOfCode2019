#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

const X_MAX: usize = 4;
const Y_MAX: usize = 4;
const X_CENTER: usize = X_MAX / 2;
const Y_CENTER: usize = Y_MAX / 2;

fn main() {
    challenge_1();
    challenge_2();
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

fn challenge_2() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut state = RecursiveState::new(&input);
    for _ in 0..200 {
        state = state.next();
    }
    println!(
        "Challenge 2: Number of bugs after 200 minutes = {}",
        state.bugs()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellState {
    Infested,
    Empty,
    Grid,
}

use CellState::*;

impl Into<char> for &CellState {
    fn into(self) -> char {
        match self {
            Infested => '#',
            Empty => '.',
            Grid => '?',
        }
    }
}

impl From<char> for CellState {
    fn from(c: char) -> Self {
        match c {
            '#' => Infested,
            '.' => Empty,
            '?' => Grid,
            _ => unimplemented!("unknown CellState: {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    tiles: Vec<Vec<CellState>>,
}

impl State {
    fn new(s: &str) -> Self {
        let tiles: Vec<Vec<_>> = s
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        Self { tiles }
    }

    fn next(&self) -> Self {
        let mut new_tiles: Vec<Vec<_>> = (0..=Y_MAX).map(|_| [Empty].repeat(X_MAX + 1)).collect();
        for (y, row) in new_tiles.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = self.next_cell_state(x, y);
            }
        }
        Self { tiles: new_tiles }
    }

    fn next_recursive(&self, inner: Option<&State>, outer: Option<&State>) -> Self {
        let mut new_tiles: Vec<Vec<_>> = (0..=Y_MAX).map(|_| [Empty].repeat(X_MAX + 1)).collect();
        for (y, row) in new_tiles.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = self.next_cell_state_recursive(x, y, inner, outer);
            }
        }
        Self { tiles: new_tiles }
    }

    fn next_cell_state(&self, x: usize, y: usize) -> CellState {
        let mut neighbours_infested = 0;
        for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x_neighbour = x as isize + dx;
            let y_neighbour = y as isize + dy;
            if x_neighbour < 0
                || y_neighbour < 0
                || x_neighbour > X_MAX as isize
                || y_neighbour > Y_MAX as isize
            {
                continue;
            }
            if let Infested = self.tiles[y_neighbour as usize][x_neighbour as usize] {
                neighbours_infested += 1;
            }
        }
        match (self.tiles[y][x], neighbours_infested) {
            (Infested, 1) | (Empty, 1) | (Empty, 2) => Infested,
            (Grid, _) => Grid,
            _ => Empty,
        }
    }

    fn next_cell_state_recursive(
        &self,
        x: usize,
        y: usize,
        inner: Option<&State>,
        outer: Option<&State>,
    ) -> CellState {
        let mut neighbours_infested = 0;
        for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x_neighbour = x as isize + dx;
            let y_neighbour = y as isize + dy;
            if x_neighbour >= 0
                && y_neighbour >= 0
                && x_neighbour <= X_MAX as isize
                && y_neighbour <= Y_MAX as isize
                && !(x_neighbour == X_CENTER as isize && y_neighbour == Y_CENTER as isize)
            {
                neighbours_infested += self.cell_bugs(x_neighbour as usize, y_neighbour as usize);
                continue;
            }
            if x_neighbour == X_CENTER as isize && y_neighbour == Y_CENTER as isize {
                if let Some(inner) = inner {
                    neighbours_infested += match x.cmp(&X_CENTER) {
                        std::cmp::Ordering::Less => inner.outer_grid_bugs(Side::Left),
                        std::cmp::Ordering::Greater => inner.outer_grid_bugs(Side::Right),
                        std::cmp::Ordering::Equal => 0,
                    };
                    neighbours_infested += match y.cmp(&Y_CENTER) {
                        std::cmp::Ordering::Less => inner.outer_grid_bugs(Side::Top),
                        std::cmp::Ordering::Greater => inner.outer_grid_bugs(Side::Bottom),
                        std::cmp::Ordering::Equal => 0,
                    };
                }
            } else if let Some(outer) = outer {
                if x_neighbour < 0 {
                    neighbours_infested += outer.inner_grid_bugs(Side::Left);
                } else if x_neighbour > X_MAX as isize {
                    neighbours_infested += outer.inner_grid_bugs(Side::Right);
                }
                if y_neighbour < 0 {
                    neighbours_infested += outer.inner_grid_bugs(Side::Top);
                } else if y_neighbour > Y_MAX as isize {
                    neighbours_infested += outer.inner_grid_bugs(Side::Bottom);
                }
            }
        }
        match (self.tiles[y][x], neighbours_infested) {
            (Infested, 1) | (Empty, 1) | (Empty, 2) => Infested,
            (Grid, _) => Grid,
            _ => Empty,
        }
    }

    fn bugs(&self) -> u64 {
        self.tiles
            .iter()
            .flatten()
            .map(|cell| if let Infested = cell { 1 } else { 0 })
            .sum()
    }

    fn inner_grid_bugs(&self, side: Side) -> u64 {
        let (x, y) = match side {
            Side::Left => (X_CENTER - 1, Y_CENTER),
            Side::Right => (X_CENTER + 1, Y_CENTER),
            Side::Top => (X_CENTER, Y_CENTER - 1),
            Side::Bottom => (X_CENTER, Y_CENTER + 1),
        };
        self.cell_bugs(x, y)
    }

    fn outer_grid_bugs(&self, side: Side) -> u64 {
        let cells: Vec<_> = match side {
            Side::Left => (0..=Y_MAX).map(|y| (0, y)).collect(),
            Side::Right => (0..=Y_MAX).map(|y| (X_MAX, y)).collect(),
            Side::Top => (0..=X_MAX).map(|x| (x, 0)).collect(),
            Side::Bottom => (0..=X_MAX).map(|x| (x, Y_MAX)).collect(),
        };
        cells.iter().map(|&(x, y)| self.cell_bugs(x, y)).sum()
    }

    fn cell_bugs(&self, x: usize, y: usize) -> u64 {
        if let Infested = self.tiles[y][x] {
            1
        } else {
            0
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
                Grid => unimplemented!(),
            })
            .sum()
    }
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
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

#[derive(Debug, Clone)]
struct RecursiveState {
    grids: HashMap<isize, State>,
    min_i: isize,
    max_i: isize,
}

impl RecursiveState {
    fn new(s: &str) -> Self {
        let mut grids: HashMap<isize, State> = Default::default();
        let mut grid = State::new(s);
        grid.tiles[Y_CENTER][X_CENTER] = Grid;
        grids.insert(0, grid);
        Self {
            grids,
            min_i: 0,
            max_i: 0,
        }
    }

    fn level(&self, i: isize) -> Option<&State> {
        self.grids.get(&i)
    }

    fn set_level(&mut self, i: isize, state: State) {
        self.grids.insert(i, state);
        if i > self.max_i {
            self.max_i = i;
        }
        if i < self.min_i {
            self.min_i = i;
        }
    }

    fn next(&self) -> Self {
        let mut state = self.to_owned();
        for (i, grid) in self.grids.clone() {
            let grid = grid.next_recursive(self.level(i + 1), self.level(i - 1));
            state.set_level(i, grid);
        }
        let inner_grid = self.level(self.max_i);
        if inner_grid.unwrap().bugs() > 0 {
            let new_grid = State::new(
                "
.....
.....
..?..
.....
.....
",
            );
            let new_grid = new_grid.next_recursive(None, inner_grid);
            state.set_level(self.max_i + 1, new_grid);
        }

        let outer_grid = self.level(self.min_i);
        if outer_grid.unwrap().bugs() > 0 {
            let new_grid = State::new(
                "
.....
.....
..?..
.....
.....
",
            );
            let new_grid = new_grid.next_recursive(outer_grid, None);
            state.set_level(self.min_i - 1, new_grid);
        }
        state
    }

    fn bugs(&self) -> u64 {
        self.grids.values().map(|g| g.bugs()).sum()
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

        let expected = "\
#..#.
####.
###.#
##.##
.##..
";
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "\
#####
....#
....#
...#.
#.###
";
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "\
#....
####.
...##
#.##.
.##.#
";
        let state = state.next();
        assert_eq!(state.to_string(), expected);

        let expected = "\
####.
....#
##..#
.....
##...
";
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
        let expected = "\
.....
.....
.....
#....
.#...
";
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

    #[test]
    fn evolves_state_for_recursive_space() {
        let state = RecursiveState::new(
            "
.....
.....
..?..
.....
.....
",
        );
        assert_eq!(
            state.next().level(0).unwrap().to_string(),
            "\
.....
.....
..?..
.....
.....
"
        );

        let state = RecursiveState::new(
            "
.....
.....
..?..
.....
.#...
",
        );
        assert_eq!(
            state.next().level(-1).unwrap().to_string(),
            "\
.....
.....
..?..
..#..
.....
"
        );
    }

    #[test]
    fn evolves_state_for_recursive_space_example() {
        let mut state = RecursiveState::new(
            "
....#
#..#.
#.?##
..#..
#....
",
        );
        for _ in 0..10 {
            state = state.next();
        }
        assert_eq!(
            state.level(-5).unwrap().to_string(),
            "\
..#..
.#.#.
..?.#
.#.#.
..#..
"
        );
        assert_eq!(
            state.level(-4).unwrap().to_string(),
            "\
...#.
...##
..?..
...##
...#.
"
        );
        assert_eq!(
            state.level(-3).unwrap().to_string(),
            "\
#.#..
.#...
..?..
.#...
#.#..
"
        );
        assert_eq!(
            state.level(-2).unwrap().to_string(),
            "\
.#.##
....#
..?.#
...##
.###.
"
        );
        assert_eq!(
            state.level(-1).unwrap().to_string(),
            "\
#..##
...##
..?..
...#.
.####
"
        );
        assert_eq!(
            state.level(0).unwrap().to_string(),
            "\
.#...
.#.##
.#?..
.....
.....
"
        );
        assert_eq!(
            state.level(1).unwrap().to_string(),
            "\
.##..
#..##
..?.#
##.##
#####
"
        );
        assert_eq!(
            state.level(2).unwrap().to_string(),
            "\
###..
##.#.
#.?..
.#.##
#.#..
"
        );
        assert_eq!(
            state.level(3).unwrap().to_string(),
            "\
..###
.....
#.?..
#....
#...#
"
        );
        assert_eq!(
            state.level(4).unwrap().to_string(),
            "\
.###.
#..#.
#.?..
##.#.
.....
"
        );
        assert_eq!(
            state.level(5).unwrap().to_string(),
            "\
####.
#..#.
#.?#.
####.
.....
"
        );
    }

    #[test]
    fn calculates_total_bugs_in_recursive_state() {
        let mut state = RecursiveState::new(
            "
....#
#..#.
#.?##
..#..
#....
",
        );
        for _ in 0..10 {
            state = state.next();
        }
        assert_eq!(state.bugs(), 99);
    }
}
