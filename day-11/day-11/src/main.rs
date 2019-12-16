use intcode_computer::Computer;
use std::collections::HashMap;

fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    let robot = run(0);
    println!("Challenge 1: Panels painted = {}", robot.num_painted());
}

fn challenge_2() {
    let robot = run(1);
    let min_x = robot.painted.keys().map(|(x, _)| *x).min().unwrap();
    let max_x = robot.painted.keys().map(|(x, _)| *x).max().unwrap();
    let min_y = robot.painted.keys().map(|(_, y)| *y).min().unwrap();
    let max_y = robot.painted.keys().map(|(_, y)| *y).max().unwrap();
    println!("Challenge 2:");
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let c = match robot.painted.get(&(x, y)).unwrap_or(&0) {
                1 => '█',
                0 => ' ',
                _ => unreachable!(),
            };
            print!("{}", c);
        }
        println!();
    }
}

fn run(starting_panel_color: u8) -> Robot {
    let mut computer = Computer::load_from_file("input");
    let mut robot = Robot::init(starting_panel_color);
    loop {
        let (out, complete) = computer.run(vec![robot.color_at_position() as i64]);
        if !out.is_empty() {
            robot.next(out[0] as u8, out[1] as u8);
        }
        if complete {
            break;
        }
    }
    robot
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

impl Direction {
    fn turn(&self, way: u8) -> Self {
        match way {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => unreachable!(),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
    painted: HashMap<(i32, i32), u8>,
    starting_panel_color: u8,
}

impl Robot {
    fn init(starting_panel_color: u8) -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Up,
            painted: Default::default(),
            starting_panel_color,
        }
    }

    fn color_at_position(&self) -> u8 {
        *self
            .painted
            .get(&(self.x, self.y))
            .unwrap_or_else(|| match (self.x, self.y) {
                (0, 0) => &self.starting_panel_color,
                _ => &0,
            })
    }

    fn num_painted(&self) -> u32 {
        self.painted.len() as u32
    }

    fn next(&mut self, color: u8, turn: u8) {
        self.painted.insert((self.x, self.y), color);
        self.direction = self.direction.turn(turn);
        self.x += match self.direction {
            Left => -1,
            Right => 1,
            _ => 0,
        };
        self.y += match self.direction {
            Up => 1,
            Down => -1,
            _ => 0,
        };
    }
}

#[cfg(test)]
mod test_day_11 {
    use super::*;

    #[test]
    fn robot_processes_instructions() {
        let mut robot = Robot::init(0);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction, Direction::Up);
        assert_eq!(robot.num_painted(), 0);
        assert_eq!(robot.color_at_position(), 0);

        robot.next(1, 0);
        assert_eq!(robot.x, -1);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction, Direction::Left);
        assert_eq!(robot.num_painted(), 1);
        assert_eq!(robot.color_at_position(), 0);

        robot.next(0, 0);
        assert_eq!(robot.x, -1);
        assert_eq!(robot.y, -1);
        assert_eq!(robot.direction, Direction::Down);
        assert_eq!(robot.num_painted(), 2);
        assert_eq!(robot.color_at_position(), 0);

        robot.next(1, 0);
        robot.next(1, 0);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction, Direction::Up);
        assert_eq!(robot.num_painted(), 4);
        assert_eq!(robot.color_at_position(), 1);

        robot.next(0, 1);
        robot.next(1, 0);
        robot.next(1, 0);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 1);
        assert_eq!(robot.direction, Direction::Left);
        assert_eq!(robot.num_painted(), 6);
        assert_eq!(robot.color_at_position(), 0);
    }
}
