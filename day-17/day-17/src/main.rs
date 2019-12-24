#![allow(dead_code)]
use intcode_computer::Computer;
use std::collections::HashMap;

fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    let mut computer = Computer::load_from_file("input");
    let env = Environment::load(computer.run(vec![]).0);
    print!("{}", env.to_string());
    println!(
        "Challenge 1: Sum of alignment parameters = {}",
        sum_of_alignment_params(&env.to_string())
    );
}

fn challenge_2() {
    let mut computer = Computer::load_from_file("input");
    let mut env = Environment::load(computer.run(vec![]).0);
    let path = find_path(&mut env);
    println!("{}", path);
    println!("{}", path.len());
}

#[derive(Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
    direction: Direction,
}

impl From<(usize, usize, Direction)> for Location {
    fn from(v: (usize, usize, Direction)) -> Self {
        Location {
            x: v.0,
            y: v.1,
            direction: v.2,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn steps_to(self, other: Direction) -> String {
        let diff = (other as i32 - self as i32 + 4) % 4;
        match diff {
            0 => "",
            1 => "R",
            2 => "RR",
            3 => "L",
            _ => unreachable!("steps_to"),
        }
        .to_string()
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}

impl From<char> for Direction {
    fn from(v: char) -> Self {
        match v {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Robot(Direction),
    Empty,
    Scaffold,
    Visited,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '^' | '>' | 'v' | '<' => Self::Robot(c.into()),
            '.' => Self::Empty,
            '#' => Self::Scaffold,
            'V' => Self::Visited,
            _ => unreachable!(),
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        (match self {
            Self::Robot(Direction::Up) => "^",
            Self::Robot(Direction::Down) => "v",
            Self::Robot(Direction::Left) => "<",
            Self::Robot(Direction::Right) => ">",
            Self::Empty => ".",
            Self::Scaffold => "#",
            Self::Visited => "V",
        })
        .to_owned()
    }
}

struct Environment {
    view: Vec<Vec<Tile>>,
    robot_location: Location,
}

impl Environment {
    fn load(input: Vec<i64>) -> Self {
        let view = String::from_utf8(input.iter().map(|b| *b as u8).collect()).unwrap();
        let mut view: Vec<Vec<Tile>> = view
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        let mut robot_location: Option<Location> = None;
        for (y, row) in view.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Robot(direction) = tile {
                    robot_location = Some((x, y, *direction).into());
                }
            }
        }
        let robot_location = robot_location.unwrap();
        view[robot_location.y][robot_location.x] = '#'.into();
        Self {
            view,
            robot_location,
        }
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.view[y][x] = tile;
    }

    fn x_max(&self) -> usize {
        self.view[0].len() - 1
    }

    fn y_max(&self) -> usize {
        self.view.len() - 1
    }

    fn shortest_path_to_unvisited(&self) -> Option<Vec<(usize, usize)>> {
        let mut tracker: Tracker = Default::default();
        tracker.insert(self.robot_location.x, self.robot_location.y, 0);
        'outer: loop {
            let (x, y, distance) = tracker.next()?;
            let mut next_steps = vec![];
            if x > 0 {
                next_steps.push((x - 1, y));
            }
            if x < self.x_max() {
                next_steps.push((x + 1, y));
            }
            if y > 0 {
                next_steps.push((x, y - 1));
            }
            if y < self.y_max() {
                next_steps.push((x, y + 1));
            }

            let new_distance = distance + 1;
            for (x_step, y_step) in next_steps {
                if self.is_unvisited(x_step, y_step) {
                    tracker.insert(x_step, y_step, new_distance);
                    break 'outer;
                }
                if tracker.contains(x_step, y_step) || !self.can_move_to(x_step, y_step) {
                    continue;
                }
                tracker.insert(x_step, y_step, new_distance);
            }
        }
        Some(tracker.path())
    }

    fn is_unvisited(&self, x: usize, y: usize) -> bool {
        self.view[y][x] == Tile::Scaffold
    }

    fn can_move_to(&self, x: usize, y: usize) -> bool {
        self.view[y][x] == Tile::Scaffold || self.view[y][x] == Tile::Visited
    }
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for row in &self.view {
            for tile in row {
                out.push_str(&tile.to_string());
            }
            out.push('\n');
        }
        out
    }
}

#[derive(Default)]
struct Tracker {
    index: usize,
    steps: Vec<(usize, usize)>,
    distances: HashMap<(usize, usize), usize>,
}

impl Tracker {
    fn next(&mut self) -> Option<(usize, usize, usize)> {
        if self.index >= self.steps.len() {
            return None;
        }
        let (x, y) = self.steps[self.index];
        let distance = *self.distances.get(&(x, y)).unwrap();
        self.index += 1;
        Some((x, y, distance))
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.distances.contains_key(&(x, y))
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let &(mut x, mut y) = self.steps.last().unwrap();
        let &last_distance = self.distances.get(&(x, y)).unwrap();
        let mut path = vec![(x, y)];
        for distance in (0..last_distance).rev() {
            if self.distances.get(&(x + 1, y)) == Some(&distance) {
                x += 1;
                path.push((x, y));
            } else if x > 0 && self.distances.get(&(x - 1, y)) == Some(&distance) {
                x -= 1;
                path.push((x, y));
            } else if self.distances.get(&(x, y + 1)) == Some(&distance) {
                y += 1;
                path.push((x, y));
            } else if y > 0 && self.distances.get(&(x, y - 1)) == Some(&distance) {
                y -= 1;
                path.push((x, y));
            } else {
                unreachable!("couldn't find step from {}, {}", x, y);
            }
        }
        path.reverse();
        path[1..].to_vec()
    }

    fn insert(&mut self, x: usize, y: usize, distance: usize) {
        self.steps.push((x, y));
        self.distances.insert((x, y), distance);
    }
}

fn sum_of_alignment_params(view: &str) -> usize {
    let mut total = 0;
    let map: Vec<Vec<char>> = view.lines().map(|line| line.chars().collect()).collect();
    let columns = map[0].len();
    for row in 1..(map.len() - 1) {
        for column in 1..(columns - 1) {
            if is_scaffold(&map, row, column)
                && is_scaffold(&map, row - 1, column)
                && is_scaffold(&map, row + 1, column)
                && is_scaffold(&map, row, column - 1)
                && is_scaffold(&map, row, column + 1)
            {
                total += row * column;
            }
        }
    }
    total
}

fn is_scaffold(map: &[Vec<char>], row: usize, column: usize) -> bool {
    map[row][column] == '#'
}

fn find_path(env: &mut Environment) -> String {
    env.set(env.robot_location.x, env.robot_location.y, 'V'.into());
    let mut out = String::new();
    loop {
        let path = env.shortest_path_to_unvisited();
        if path.is_none() {
            break;
        }
        for (x_new, y_new) in path.unwrap() {
            let x = env.robot_location.x;
            let y = env.robot_location.y;
            let new_direction = match (x_new as i32 - x as i32, y_new as i32 - y as i32) {
                (0, -1) => Direction::Up,
                (0, 1) => Direction::Down,
                (-1, 0) => Direction::Left,
                (1, 0) => Direction::Right,
                step => unreachable!("unexpected step: {:?}", step),
            };
            out.push_str(&env.robot_location.direction.steps_to(new_direction));
            out.push('F');
            env.set(x_new, y_new, 'V'.into());
            env.robot_location.direction = new_direction;
            env.robot_location.x = x_new;
            env.robot_location.y = y_new;
        }
    }
    out
}

#[cfg(test)]
mod test_day_17 {
    use super::*;

    fn to_vec_i64(input: &str) -> Vec<i64> {
        input.chars().map(|c| c as i64).collect()
    }

    #[test]
    fn generates_output() {
        let mut computer = Computer::load_from_file("../input");
        let out = Environment::load(computer.run(vec![]).0).to_string();
        let lines: Vec<_> = out.lines().collect();
        assert!(lines.len() > 1);
        assert!(lines[0].len() > 1);
    }

    #[test]
    fn locates_robot() {
        let env = Environment::load(to_vec_i64("^"));
        assert_eq!(env.robot_location, (0, 0, Direction::Up).into());

        let env = Environment::load(to_vec_i64(".^"));
        assert_eq!(env.robot_location, (1, 0, Direction::Up).into());

        let env = Environment::load(to_vec_i64("..\n^."));
        assert_eq!(env.robot_location, (0, 1, Direction::Up).into());

        let env = Environment::load(to_vec_i64(".>"));
        assert_eq!(env.robot_location, (1, 0, Direction::Right).into());

        let env = Environment::load(to_vec_i64(".v"));
        assert_eq!(env.robot_location, (1, 0, Direction::Down).into());

        let env = Environment::load(to_vec_i64(".<"));
        assert_eq!(env.robot_location, (1, 0, Direction::Left).into());
    }

    #[test]
    fn replaces_robot_marker_with_scaffold() {
        let env = Environment::load(to_vec_i64(".^"));
        assert_eq!(env.view[0][1], '#'.into());
    }

    #[test]
    fn calculates_sum_of_alignment_parameters() {
        let view = "
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
"
        .trim_start_matches('\n');
        assert_eq!(sum_of_alignment_params(view), 76);
    }

    #[test]
    fn finds_path_that_visits_every_piece_of_scaffold() {
        let view = to_vec_i64(
            "
..#..........
..#..........
#######......
#.#...#......
###########..
..#...#...#..
..#####...^..
"
            .trim_start_matches('\n'),
        );
        let mut env = Environment::load(view);
        let path = find_path(&mut env);
        assert_eq!(path[..7], *"FFLFFFF");
    }

    #[test]
    fn finds_path_that_when_multiple_steps_are_required_to_get_to_unvisited() {
        let view = to_vec_i64(
            "
VVV>
.#..
"
            .trim_start_matches('\n'),
        );
        let mut env = Environment::load(view);
        let path = find_path(&mut env);
        assert_eq!(path, *"RRFFLF");
    }
}
