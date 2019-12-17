use regex::{Captures, Regex};
use std::cmp::Ordering::*;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut system = Parser::parse(&input);
    apply_steps(&mut system, 1000);
    let energy = total_energy(&system);
    println!("Challenge 1: Total energy after 1000 steps = {}", energy);
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32, vx: i32, vy: i32, vz: i32) -> Self {
        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }

    fn step(&mut self, system: &[Moon]) {
        for other in system {
            self.vx += Moon::velocity_step(self.x, other.x);
            self.vy += Moon::velocity_step(self.y, other.y);
            self.vz += Moon::velocity_step(self.z, other.z);
        }
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn velocity_step(self_p: i32, other_p: i32) -> i32 {
        match other_p.cmp(&self_p) {
            Greater => 1,
            Less => -1,
            Equal => 0,
        }
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }
}

struct Parser<'a> {
    captures: Captures<'a>,
}

impl<'a> Parser<'_> {
    fn parse(input: &str) -> Vec<Moon> {
        input.trim().lines().map(Parser::parse_one).collect()
    }

    fn parse_one(line: &str) -> Moon {
        let full_regex = Regex::new(concat!(
            r"pos=<x=(?P<x>[^,]+), y=(?P<y>[^,]+), z=(?P<z>[^>]+)>.*",
            r"vel=<x=(?P<vx>[^,]+), y=(?P<vy>[^,]+), z=(?P<vz>[^>]+)>"
        ))
        .unwrap();
        let pos_only_regex =
            Regex::new(r"<x=(?P<x>[^,]+), y=(?P<y>[^,]+), z=(?P<z>[^>]+)>").unwrap();
        let captures = full_regex
            .captures(line)
            .unwrap_or_else(|| pos_only_regex.captures(line).unwrap());
        let p = Parser { captures };
        Moon::new(
            p.get("x").unwrap(),
            p.get("y").unwrap(),
            p.get("z").unwrap(),
            p.get("vx").unwrap_or(0),
            p.get("vy").unwrap_or(0),
            p.get("vz").unwrap_or(0),
        )
    }

    fn get(&self, name: &str) -> Option<i32> {
        self.captures
            .name(name)
            .map(|v| v.as_str().trim().parse().unwrap())
    }
}

fn step(system: &mut Vec<Moon>) {
    let init_system = system.clone();
    for moon in system.iter_mut() {
        moon.step(&init_system);
    }
}

fn apply_steps(mut system: &mut Vec<Moon>, number_of_steps: u32) {
    for _ in 0..number_of_steps {
        step(&mut system);
    }
}

fn total_energy(system: &[Moon]) -> i32 {
    let mut energy = 0;
    for moon in system {
        energy += moon.energy();
    }
    energy
}

#[cfg(test)]
mod test_day_12 {
    use super::*;

    #[test]
    fn parses_input_with_velocities() {
        let input = r"
            pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
            pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
            pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
            pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
        ";
        let system = Parser::parse(&input);
        assert_eq!(
            system,
            vec![
                Moon::new(2, -1, 1, 3, -1, -1),
                Moon::new(3, -7, -4, 1, 3, 3),
                Moon::new(1, -7, 5, -3, 1, -3),
                Moon::new(2, 2, 0, -1, -3, 1),
            ]
        );
    }

    #[test]
    fn parses_input_without_velocities() {
        let input = r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
        ";
        let system = Parser::parse(&input);
        assert_eq!(
            system,
            vec![
                Moon::new(-1, 0, 2, 0, 0, 0),
                Moon::new(2, -10, -7, 0, 0, 0),
                Moon::new(4, -8, 8, 0, 0, 0),
                Moon::new(3, 5, -1, 0, 0, 0),
            ]
        );
    }

    #[test]
    fn applies_time_step() {
        let mut system = Parser::parse(
            r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
            ",
        );

        step(&mut system);
        assert_eq!(
            system,
            Parser::parse(
                r"
                pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
                pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
                pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
                pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
                "
            )
        );

        step(&mut system);
        assert_eq!(
            system,
            Parser::parse(
                r"
                pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
                pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
                pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
                pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>
                "
            )
        );
    }

    #[test]
    fn calculates_total_energy() {
        let system = Parser::parse(
            r"
            pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
            pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
            pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
            pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
            ",
        );
        assert_eq!(total_energy(&system), 179);
    }

    #[test]
    fn convenience_function_permits_apply_multiple_steps() {
        let mut system = Parser::parse(
            r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
            ",
        );
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                r"
                pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
                pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
                pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
                pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
                "
            )
        );
    }

    #[test]
    fn example_matches_specification() {
        let mut system = Parser::parse(
            r"
            <x=-8, y=-10, z=0>
            <x=5, y=5, z=10>
            <x=2, y=-7, z=3>
            <x=9, y=-8, z=-3>
            ",
        );

        // After 0 steps:
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= -8, y=-10, z=  0>, vel=<x=  0, y=  0, z=  0>
                pos=<x=  5, y=  5, z= 10>, vel=<x=  0, y=  0, z=  0>
                pos=<x=  2, y= -7, z=  3>, vel=<x=  0, y=  0, z=  0>
                pos=<x=  9, y= -8, z= -3>, vel=<x=  0, y=  0, z=  0>
                "
            )
        );

        // After 10 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= -9, y=-10, z=  1>, vel=<x= -2, y= -2, z= -1>
                pos=<x=  4, y= 10, z=  9>, vel=<x= -3, y=  7, z= -2>
                pos=<x=  8, y=-10, z= -3>, vel=<x=  5, y= -1, z= -2>
                pos=<x=  5, y=-10, z=  3>, vel=<x=  0, y= -4, z=  5>
                "
            )
        );

        // After 20 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x=-10, y=  3, z= -4>, vel=<x= -5, y=  2, z=  0>
                pos=<x=  5, y=-25, z=  6>, vel=<x=  1, y=  1, z= -4>
                pos=<x= 13, y=  1, z=  1>, vel=<x=  5, y= -2, z=  2>
                pos=<x=  0, y=  1, z=  7>, vel=<x= -1, y= -1, z=  2>
                "
            )
        );

        // After 30 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= 15, y= -6, z= -9>, vel=<x= -5, y=  4, z=  0>
                pos=<x= -4, y=-11, z=  3>, vel=<x= -3, y=-10, z=  0>
                pos=<x=  0, y= -1, z= 11>, vel=<x=  7, y=  4, z=  3>
                pos=<x= -3, y= -2, z=  5>, vel=<x=  1, y=  2, z= -3>
                "
            )
        );

        // After 40 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= 14, y=-12, z= -4>, vel=<x= 11, y=  3, z=  0>
                pos=<x= -1, y= 18, z=  8>, vel=<x= -5, y=  2, z=  3>
                pos=<x= -5, y=-14, z=  8>, vel=<x=  1, y= -2, z=  0>
                pos=<x=  0, y=-12, z= -2>, vel=<x= -7, y= -3, z= -3>
                "
            )
        );

        // After 50 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x=-23, y=  4, z=  1>, vel=<x= -7, y= -1, z=  2>
                pos=<x= 20, y=-31, z= 13>, vel=<x=  5, y=  3, z=  4>
                pos=<x= -4, y=  6, z=  1>, vel=<x= -1, y=  1, z= -3>
                pos=<x= 15, y=  1, z= -5>, vel=<x=  3, y= -3, z= -3>
                "
            )
        );

        // After 60 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= 36, y=-10, z=  6>, vel=<x=  5, y=  0, z=  3>
                pos=<x=-18, y= 10, z=  9>, vel=<x= -3, y= -7, z=  5>
                pos=<x=  8, y=-12, z= -3>, vel=<x= -2, y=  1, z= -7>
                pos=<x=-18, y= -8, z= -2>, vel=<x=  0, y=  6, z= -1>
                "
            )
        );

        // After 70 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x=-33, y= -6, z=  5>, vel=<x= -5, y= -4, z=  7>
                pos=<x= 13, y= -9, z=  2>, vel=<x= -2, y= 11, z=  3>
                pos=<x= 11, y= -8, z=  2>, vel=<x=  8, y= -6, z= -7>
                pos=<x= 17, y=  3, z=  1>, vel=<x= -1, y= -1, z= -3>
                "
            )
        );

        // After 80 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x= 30, y= -8, z=  3>, vel=<x=  3, y=  3, z=  0>
                pos=<x= -2, y= -4, z=  0>, vel=<x=  4, y=-13, z=  2>
                pos=<x=-18, y= -7, z= 15>, vel=<x= -8, y=  2, z= -2>
                pos=<x= -2, y= -1, z= -8>, vel=<x=  1, y=  8, z=  0>
                "
            )
        );

        // After 90 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x=-25, y= -1, z=  4>, vel=<x=  1, y= -3, z=  4>
                pos=<x=  2, y= -9, z=  0>, vel=<x= -3, y= 13, z= -1>
                pos=<x= 32, y= -8, z= 14>, vel=<x=  5, y= -4, z=  6>
                pos=<x= -1, y= -2, z= -8>, vel=<x= -3, y= -6, z= -9>
                "
            )
        );

        // After 100 steps:
        apply_steps(&mut system, 10);
        assert_eq!(
            system,
            Parser::parse(
                "
                pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>
                pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>
                pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>
                pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>
                "
            )
        );

        assert_eq!(total_energy(&system), 1940);
    }
}
