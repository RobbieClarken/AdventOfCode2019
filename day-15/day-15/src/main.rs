use intcode_computer::Computer;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;
const X_OFFSET: usize = COLUMNS / 2;
const Y_OFFSET: usize = ROWS / 2;
const DRAW: bool = false;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut computer = Computer::load_from_file("input");
    let mut robot = Robot::new();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    loop {
        let direction = robot.next_move();
        if direction.is_none() {
            break;
        }
        let direction = direction.unwrap();
        let (out, _) = computer.run(vec![direction.into()]);
        robot.process(out[0].into());
        if DRAW {
            handle.write_all(b"\x1b[2J").unwrap();
            handle.write_all(robot.map().as_bytes()).unwrap();
            sleep(Duration::from_millis(10));
        }
    }
    let path = PathFinder::path_to_tile((0, 0), Tile::Target, &robot.environment).unwrap();
    println!("Challenge 1: Moves to oxygen system = {}", path.len());
}

#[derive(Clone, Copy)]
enum Bounds {
    Unset,
    Set((usize, usize), (usize, usize)),
}

impl Bounds {
    fn include(self, row: usize, column: usize) -> Self {
        match self {
            Self::Unset => Self::Set((row, row), (column, column)),
            Self::Set((row_min, row_max), (col_min, col_max)) => Self::Set(
                (min(row, row_min), max(row, row_max)),
                (min(column, col_min), max(column, col_max)),
            ),
        }
    }

    fn unwrap(&self) -> ((usize, usize), (usize, usize)) {
        match *self {
            Self::Unset => panic!(),
            Self::Set(row_bounds, col_bounds) => (row_bounds, col_bounds),
        }
    }
}

struct Environment {
    tiles: Box<[[Tile; COLUMNS]; ROWS]>,
    bounds: Bounds,
}

impl Environment {
    fn new() -> Self {
        let tiles = Box::new([[Tile::Unknown; COLUMNS]; ROWS]);
        Self {
            tiles,
            bounds: Bounds::Unset,
        }
    }

    #[allow(dead_code)]
    fn from_map(map: &str) -> Self {
        let mut environment = Self::new();
        for (y, line) in map.trim_start_matches('\n').lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                let tile: Tile = chr.into();
                environment.set(x as i32, y as i32, tile);
            }
        }
        environment
    }

    fn get(&self, x: i32, y: i32) -> Tile {
        self.tiles[Self::row(y)][Self::column(x)]
    }

    fn set(&mut self, x: i32, y: i32, tile: Tile) {
        let row = Self::row(y);
        let col = Self::column(x);
        self.bounds = self.bounds.include(row, col);
        self.tiles[row][col] = tile;
    }

    fn row(y: i32) -> usize {
        (y + Y_OFFSET as i32) as usize
    }

    fn column(x: i32) -> usize {
        (x + X_OFFSET as i32) as usize
    }

    fn map(&self) -> String {
        let ((row_min, row_max), (col_min, col_max)) = self.bounds.unwrap();
        let mut out = Vec::new();
        for row in row_min..=row_max {
            for col in col_min..=col_max {
                write!(&mut out, "{}", self.tiles[row][col]).unwrap();
            }
            writeln!(&mut out).unwrap();
        }
        String::from_utf8(out).unwrap()
    }
}

struct PathFinder<'a> {
    environment: &'a Environment,
    paths: Vec<(i32, i32)>,
    visited: HashMap<(i32, i32), usize>,
}

impl<'a> PathFinder<'_> {
    fn path_to_tile(
        start: (i32, i32),
        tile: Tile,
        environment: &'a Environment,
    ) -> Option<Vec<(i32, i32)>> {
        let paths = vec![start];
        let mut visited = HashMap::new();
        visited.insert(start, 0);
        let mut finder = PathFinder {
            environment,
            paths,
            visited,
        };
        finder.find(|(x, y)| environment.get(x, y) == tile)
    }

    fn find<F>(&mut self, at_destination: F) -> Option<Vec<(i32, i32)>>
    where
        F: Fn((i32, i32)) -> bool,
    {
        let mut i = 0;
        let mut found = None;
        'outer: while i < self.paths.len() {
            let (x, y) = self.paths[i];
            let mut new_steps = Vec::new();
            if self.is_new_visitable(x, y - 1) {
                new_steps.push((x, y - 1));
            }
            if self.is_new_visitable(x + 1, y) {
                new_steps.push((x + 1, y));
            }
            if self.is_new_visitable(x, y + 1) {
                new_steps.push((x, y + 1));
            }
            if self.is_new_visitable(x - 1, y) {
                new_steps.push((x - 1, y));
            }
            let distance_of_new_steps = *self.visited.get(&(x, y)).unwrap() + 1;
            for (x, y) in new_steps {
                self.paths.push((x, y));
                self.visited.insert((x, y), distance_of_new_steps);
                if at_destination((x, y)) {
                    found = Some(((x, y), distance_of_new_steps));
                    break 'outer;
                }
            }
            i += 1;
        }
        let ((mut x, mut y), target_distance) = found?;
        let mut path: VecDeque<_> = vec![(x, y)].into();
        for next_distance in (1..target_distance).rev() {
            if self.visited.get(&(x, y - 1)) == Some(&next_distance) {
                y -= 1;
                path.push_front((x, y));
                continue;
            }
            if self.visited.get(&(x + 1, y)) == Some(&next_distance) {
                x += 1;
                path.push_front((x, y));
                continue;
            }
            if self.visited.get(&(x, y + 1)) == Some(&next_distance) {
                y += 1;
                path.push_front((x, y));
                continue;
            }
            if self.visited.get(&(x - 1, y)) == Some(&next_distance) {
                x -= 1;
                path.push_front((x, y));
                continue;
            }
        }
        Some(path.into())
    }

    fn is_new_visitable(&self, x: i32, y: i32) -> bool {
        !self.visited.contains_key(&(x, y)) && self.environment.get(x, y) != Tile::Wall
    }
}

struct Robot {
    environment: Environment,
    x: i32,
    y: i32,
}

impl Robot {
    fn new() -> Self {
        let mut environment = Environment::new();
        environment.set(0, 0, Tile::Empty);
        Self {
            environment,
            x: 0,
            y: 0,
        }
    }

    fn next_move(&self) -> Option<Direction> {
        let path = PathFinder::path_to_tile((self.x, self.y), Tile::Unknown, &self.environment)?;
        let (x_next, y_next) = path[0];
        Some(match (x_next - self.x, y_next - self.y) {
            (0, -1) => Direction::North,
            (1, 0) => Direction::East,
            (0, 1) => Direction::South,
            (-1, 0) => Direction::West,
            _ => unreachable!(),
        })
    }

    fn process(&mut self, status: Status) {
        let (dx, dy) = self.next_move().unwrap().step();
        match status {
            Status::HitWall => {
                self.environment.set(self.x + dx, self.y + dy, Tile::Wall);
            }
            Status::MovedToEmpty => {
                self.x += dx;
                self.y += dy;
                self.environment.set(self.x, self.y, Tile::Empty);
            }
            Status::MovedToTarget => {
                self.x += dx;
                self.y += dy;
                self.environment.set(self.x, self.y, Tile::Target);
            }
        }
    }

    fn map(&self) -> String {
        self.environment.map()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Empty,
    Unknown,
    Wall,
    Target,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            ' ' => Self::Unknown,
            '#' => Self::Wall,
            'X' => Self::Target,
            _ => unimplemented!("unexpected tile character: {}", c),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Unknown => ' ',
                Self::Wall => '#',
                Self::Target => 'X',
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy, ToPrimitive)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        self.to_i64().unwrap()
    }
}

impl Direction {
    fn step(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, ToPrimitive, FromPrimitive)]
enum Status {
    HitWall = 0,
    MovedToEmpty = 1,
    MovedToTarget = 2,
}

impl From<i64> for Status {
    fn from(v: i64) -> Self {
        Status::from_i64(v).unwrap()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_day_15 {
    use super::*;

    #[test]
    fn directions_convert_to_right_instructions() {
        assert_eq!(Direction::North as i64, 1);
        assert_eq!(Direction::South as i64, 2);
        assert_eq!(Direction::West as i64, 3);
        assert_eq!(Direction::East as i64, 4);
    }

    #[test]
    fn robot_starts_on_empty_space() {
        let robot = Robot::new();
        assert_eq!(robot.environment.get(0, 0), Tile::Empty);
    }

    #[test]
    fn robot_reports_unexplored_cells_as_Unknown() {
        let robot = Robot::new();
        assert_eq!(robot.environment.get(1, 0), Tile::Unknown);
    }

    #[test]
    fn robot_starts_moving_north() {
        let robot = Robot::new();
        assert_eq!(robot.next_move(), Some(Direction::North));
    }

    #[test]
    fn robot_processes_status_HitWall() {
        let mut robot = Robot::new();
        robot.next_move();
        robot.process(Status::HitWall);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.environment.get(0, -1), Tile::Wall);

        let mut robot = Robot::new();
        robot.next_move();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        assert_eq!(robot.environment.get(0, -2), Tile::Wall);
    }

    #[test]
    fn robot_processes_status_MovedToEmpty() {
        let mut robot = Robot::new();
        robot.next_move();
        robot.process(Status::MovedToEmpty);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, -1);
        assert_eq!(robot.environment.get(0, -1), Tile::Empty);

        assert_eq!(robot.next_move(), Some(Direction::North));
        robot.process(Status::MovedToEmpty);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, -2);
        assert_eq!(robot.environment.get(0, -2), Tile::Empty);
    }

    #[test]
    fn robot_processes_status_MovedToTarget() {
        let mut robot = Robot::new();
        robot.next_move();
        robot.process(Status::MovedToTarget);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, -1);
        assert_eq!(robot.environment.get(0, -1), Tile::Target);
    }

    #[test]
    fn robot_keeps_going_North_until_hits_a_wall() {
        let mut robot = Robot::new();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        let expected_map = "\
#
.
.
.
.
";
        assert_eq!(robot.map(), expected_map);
    }

    #[test]
    fn robot_goes_around_walls() {
        let mut robot = Robot::new();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        let expected_map = "
 #
 .
#.
..
. \n\
"
        .trim_start_matches('\n');
        assert_eq!(robot.map(), expected_map);
    }

    #[test]
    fn robot_goes_south_to_avoid_obstacles() {
        let mut robot = Robot::new();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        let expected_map = "
### .
...#.
. ...
"
        .trim_start_matches('\n');
        assert_eq!(robot.map(), expected_map);
    }

    #[test]
    fn robot_goes_west_to_get_around_walls() {
        let mut robot = Robot::new();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        let expected_map = "
 # \n\
..#
 . \n\
"
        .trim_start_matches('\n');
        assert_eq!(robot.map(), expected_map);
    }

    #[test]
    fn robot_goes_to_nearest_Unknown_when_surrounded_by_Known() {
        let mut robot = Robot::new();
        robot.process(Status::MovedToEmpty);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::MovedToEmpty);
        robot.process(Status::MovedToEmpty);
        let expected_map = "
 # \n\
#.#
 ..
"
        .trim_start_matches('\n');
        println!("{}", robot.map());
        assert_eq!(robot.map(), expected_map);
    }

    #[test]
    fn PathFinder_finds_path_to_closest_Unknown() {
        let environment = Environment::from_map(
            "
 # \n\
#.#
 . \n\
",
        );
        assert_eq!(
            PathFinder::path_to_tile((1, 1), Tile::Unknown, &environment),
            Some(vec![(1, 2), (2, 2)]),
        );
    }

    #[test]
    fn robot_returns_next_move_as_None_if_no_unvisited_cells() {
        let mut robot = Robot::new();
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        robot.process(Status::HitWall);
        assert_eq!(robot.next_move(), None);
    }

    #[test]
    fn PathFinder_finds_path_to_Target() {
        let environment = Environment::from_map(
            "
######
#.#.X#
#.#..#
#....#
######
",
        );
        assert_eq!(
            PathFinder::path_to_tile((1, 1), Tile::Target, &environment)
                .unwrap()
                .len(),
            7
        );
    }
}
