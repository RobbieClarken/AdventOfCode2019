use intcode_computer::Computer;
use std::collections::HashSet;

const MAX_PATTERN_LENGTH: usize = 20;

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
    let pattern_sets = get_pattern_sets(&[path], 3);
    let best_set = pattern_sets
        .iter()
        .max_by_key(|set| set[0].len() + set[1].len() + set[2].len());
    println!("{:?}", best_set);
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
    fn left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn step_from(self, x: i32, y: i32) -> (i32, i32) {
        let (dx, dy) = match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        };
        (x + dx, y + dy)
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
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '^' | '>' | 'v' | '<' => Self::Robot(c.into()),
            '.' => Self::Empty,
            '#' => Self::Scaffold,
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

    fn x_max(&self) -> usize {
        self.view[0].len() - 1
    }

    fn y_max(&self) -> usize {
        self.view.len() - 1
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x <= self.x_max() as i32 && y <= self.y_max() as i32
    }

    fn is_scaffold(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        let tile = self.view[y as usize][x as usize];
        tile == Tile::Scaffold
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
    let mut x = env.robot_location.x as i32;
    let mut y = env.robot_location.y as i32;
    let mut direction = env.robot_location.direction;
    let mut out = String::new();
    loop {
        let (x_new, y_new) = direction.step_from(x, y);
        if env.is_scaffold(x_new, y_new) {
            x = x_new;
            y = y_new;
            out.push('F');
            continue;
        }

        let direction_new = direction.left();
        let (x_new, y_new) = direction_new.step_from(x, y);
        if env.is_scaffold(x_new, y_new) {
            x = x_new;
            y = y_new;
            direction = direction_new;
            out.push_str("LF");
            continue;
        }

        let direction_new = direction.right();
        let (x_new, y_new) = direction_new.step_from(x, y);
        if env.is_scaffold(x_new, y_new) {
            x = x_new;
            y = y_new;
            direction = direction_new;
            out.push_str("RF");
            continue;
        }

        break;
    }
    out
}

fn get_pattern_sets(slices: &[String], n: usize) -> Vec<Vec<String>> {
    if slices.is_empty() {
        return vec![vec!["".to_owned(); n]];
    }
    if n == 1 {
        let smallest = slices.iter().min_by_key(|s| s.len()).unwrap();
        if reduce_pattern(smallest).len() > MAX_PATTERN_LENGTH {
            return vec![];
        }
        for slice in slices {
            let leftover = slice.replace(smallest, "");
            if !leftover.is_empty() {
                return vec![];
            }
        }
        return vec![vec![smallest.to_string()]];
    } else {
        let mut pattern_sets = HashSet::new();
        for slice in slices {
            for length in 1..=slice.len() {
                let candidate = slice[..length].to_owned();
                if reduce_pattern(&candidate).len() > MAX_PATTERN_LENGTH {
                    break;
                }
                let new_slices: Vec<_> = slices.iter().fold(vec![], |mut acc, s| {
                    acc.extend(
                        s.split(&candidate)
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_owned()),
                    );
                    acc
                });
                for mut pattern_set in get_pattern_sets(&new_slices, n - 1) {
                    pattern_set.push(candidate.clone());
                    pattern_set.sort();
                    pattern_sets.insert(pattern_set);
                }
            }
        }
        let mut pattern_sets: Vec<Vec<String>> = pattern_sets.iter().cloned().collect();
        pattern_sets.sort();
        pattern_sets
    }
}

fn reduce_pattern(pattern: &str) -> String {
    let mut out = String::new();
    let mut counter: Option<usize> = None;
    for c in pattern.chars() {
        match (c, &mut counter) {
            ('F', None) => {
                counter = Some(1);
            }
            ('F', Some(count)) => {
                *count += 1;
            }
            (_, None) => {
                out.push(c);
            }
            (_, Some(count)) => {
                out.push_str(&format!("{}", count));
                out.push(c);
                counter = None;
            }
        }
    }
    if let Some(count) = counter {
        out.push_str(&format!("{}", count));
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
        assert_eq!(&path, "FFLFFFFFFFFFFRFFRFFFFFFRFFFFRFFFFRFFFFFF");
    }

    fn vec_of_strings(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn finds_patterns_to_cover_slices() {
        let slices = vec![];
        assert_eq!(get_pattern_sets(&slices, 1), vec![vec!["".to_owned()]]);

        let slices = vec_of_strings(&["R"]);
        assert_eq!(get_pattern_sets(&slices, 1), vec![vec!["R".to_owned()]]);

        let slices = vec_of_strings(&["RRR", "R", "RR"]);
        assert_eq!(get_pattern_sets(&slices, 1), vec![vec!["R".to_owned()]]);

        let slices = vec_of_strings(&["R", "L"]);
        assert_eq!(get_pattern_sets(&slices, 1), Vec::<Vec<String>>::new());

        let slices = vec_of_strings(&["R", "LLL", "RR", "L"]);
        assert_eq!(
            get_pattern_sets(&slices, 2),
            vec![vec_of_strings(&["L", "R"]),]
        );

        let slices = vec_of_strings(&["RF", "LF"]);
        assert_eq!(
            get_pattern_sets(&slices, 2),
            vec![vec_of_strings(&["LF", "RF"])]
        );

        let slices = vec_of_strings(&["RF", "RFLF", "LF"]);
        assert_eq!(
            get_pattern_sets(&slices, 2),
            vec![vec_of_strings(&["LF", "RF"])]
        );

        let slices = vec_of_strings(&["RL", "LRLF", "LF"]);
        assert_eq!(get_pattern_sets(&slices, 2), Vec::<Vec<String>>::new());

        let slices = vec_of_strings(&["R"]);
        assert_eq!(
            get_pattern_sets(&slices, 3),
            vec![vec_of_strings(&["", "", "R"])]
        );

        let slices = vec_of_strings(&["RLF"]);
        assert_eq!(
            get_pattern_sets(&slices, 2),
            vec![
                vec_of_strings(&["", "RLF"]),
                vec_of_strings(&["F", "RL"]),
                vec_of_strings(&["LF", "R"]),
            ]
        );
    }

    #[test]
    fn allow_reduced_patterns_up_to_20() {
        let slices = vec_of_strings(&["RFRFRFRFRFRFRFRFRFRF"]);
        assert_eq!(
            get_pattern_sets(&slices, 1),
            vec![vec_of_strings(&["RFRFRFRFRFRFRFRFRFRF"])]
        );

        let slices = vec_of_strings(&["RFRFRFRFRFRFRFRFRFRFF"]);
        assert_eq!(
            get_pattern_sets(&slices, 1),
            vec![vec_of_strings(&["RFRFRFRFRFRFRFRFRFRFF"])]
        );

        let slices = vec_of_strings(&["RFRFRFRFRFRFRFRFRFRFF"]);
        let sets = get_pattern_sets(&slices, 2);
        assert!(sets.contains(&vec_of_strings(&["", "RFRFRFRFRFRFRFRFRFRFF"])));
    }

    #[test]
    fn doesnt_suggest_patterns_longer_than_20_chars_when_reduced() {
        let slices = vec_of_strings(&["RFRFRFRFRFRFRFRFRFRFR"]);
        assert_eq!(get_pattern_sets(&slices, 1), Vec::<Vec<String>>::new());

        let slices = vec_of_strings(&["RFRFRFRFRFRFRFRFRFRFR"]);
        let sets = get_pattern_sets(&slices, 2);
        assert!(!sets.contains(&vec_of_strings(&["", "RFRFRFRFRFRFRFRFRFRFR"])));
    }

    #[test]
    fn reduces_pattern() {
        let pattern = "RRFLLFFFRF";
        assert_eq!(reduce_pattern(pattern), "RR1LL3R1");
    }
}
