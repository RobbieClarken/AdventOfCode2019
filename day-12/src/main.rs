use regex::{Captures, Regex};
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let system = Parser::parse(&input);
    challenge_1(&system);
    challenge_2(&system);
    // benchmark(&system);
}

#[allow(dead_code)]
fn benchmark(system: &System) {
    let mut system = system.to_owned();
    let t0 = Instant::now();
    system.apply_steps(100_000_000);
    println!("{}", t0.elapsed().as_millis());
    println!("{:?}", system);
}

fn challenge_1(system: &System) {
    let mut system = system.to_owned();
    system.apply_steps(1000);
    println!(
        "Challenge 1: Total energy after 1000 steps = {}",
        system.energy()
    );
}

fn challenge_2(system: &System) {
    println!(
        "Challenge 2: Steps before system repeats = {}",
        system.steps_until_repeat()
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct System {
    x: System1d,
    y: System1d,
    z: System1d,
}

impl System {
    fn apply_steps(&mut self, number_of_steps: u32) {
        let rx_x = self.x.apply_steps_async(number_of_steps);
        let rx_y = self.y.apply_steps_async(number_of_steps);
        let rx_z = self.z.apply_steps_async(number_of_steps);
        self.x = rx_x.recv().unwrap();
        self.y = rx_y.recv().unwrap();
        self.z = rx_z.recv().unwrap();
    }

    fn steps_until_repeat(&self) -> u64 {
        let rx_x = self.x.steps_until_repeat_async();
        let rx_y = self.y.steps_until_repeat_async();
        let rx_z = self.z.steps_until_repeat_async();
        let x_steps = rx_x.recv().unwrap();
        let y_steps = rx_y.recv().unwrap();
        let z_steps = rx_z.recv().unwrap();
        lcm(lcm(x_steps, y_steps), z_steps)
    }

    fn energy(&self) -> i32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let m1_energy = (x.0.potential_energy() + y.0.potential_energy() + z.0.potential_energy())
            * (x.0.kinetic_energy() + y.0.kinetic_energy() + z.0.kinetic_energy());
        let m2_energy = (x.1.potential_energy() + y.1.potential_energy() + z.1.potential_energy())
            * (x.1.kinetic_energy() + y.1.kinetic_energy() + z.1.kinetic_energy());
        let m3_energy = (x.2.potential_energy() + y.2.potential_energy() + z.2.potential_energy())
            * (x.2.kinetic_energy() + y.2.kinetic_energy() + z.2.kinetic_energy());
        let m4_energy = (x.3.potential_energy() + y.3.potential_energy() + z.3.potential_energy())
            * (x.3.kinetic_energy() + y.3.kinetic_energy() + z.3.kinetic_energy());
        m1_energy + m2_energy + m3_energy + m4_energy
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct System1d(Moon1d, Moon1d, Moon1d, Moon1d);

impl System1d {
    fn step(&mut self) {
        let initial = *self;
        self.0.step(&initial);
        self.1.step(&initial);
        self.2.step(&initial);
        self.3.step(&initial);
    }

    fn apply_steps_async(mut self, number_of_steps: u32) -> mpsc::Receiver<Self> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            self.apply_steps(number_of_steps);
            tx.send(self).unwrap();
        });
        rx
    }

    fn apply_steps(&mut self, number_of_steps: u32) {
        for _ in 0..number_of_steps {
            self.step();
        }
    }

    fn steps_until_repeat_async(self) -> mpsc::Receiver<u64> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || tx.send(self.steps_until_repeat()).unwrap());
        rx
    }

    fn steps_until_repeat(&self) -> u64 {
        let mut system = *self;
        let mut seen: HashSet<System1d> = Default::default();
        let mut steps = 0;
        loop {
            if !seen.insert(system) {
                break;
            }
            steps += 1;
            system.step();
        }
        steps
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Moon1d {
    p: i32,
    v: i32,
}

impl Moon1d {
    fn new(p: i32, v: i32) -> Self {
        Self { p, v }
    }

    fn step(&mut self, system: &System1d) {
        let m1 = system.0;
        let m2 = system.1;
        let m3 = system.2;
        let m4 = system.3;
        self.v += (m1.p - self.p).signum()
            + (m2.p - self.p).signum()
            + (m3.p - self.p).signum()
            + (m4.p - self.p).signum();
        self.p += self.v;
    }

    fn potential_energy(self) -> i32 {
        self.p.abs()
    }

    fn kinetic_energy(self) -> i32 {
        self.v.abs()
    }
}

struct Parser<'a> {
    captures: Captures<'a>,
}

impl<'a> Parser<'_> {
    fn parse(input: &str) -> System {
        let moons: Vec<_> = input.trim().lines().map(Parser::parse_one).collect();
        System {
            x: System1d(moons[0].0, moons[1].0, moons[2].0, moons[3].0),
            y: System1d(moons[0].1, moons[1].1, moons[2].1, moons[3].1),
            z: System1d(moons[0].2, moons[1].2, moons[2].2, moons[3].2),
        }
    }

    fn parse_one(line: &str) -> (Moon1d, Moon1d, Moon1d) {
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
        let m1 = Moon1d::new(p.get("x").unwrap(), p.get("vx").unwrap_or(0));
        let m2 = Moon1d::new(p.get("y").unwrap(), p.get("vy").unwrap_or(0));
        let m3 = Moon1d::new(p.get("z").unwrap(), p.get("vz").unwrap_or(0));
        (m1, m2, m3)
    }

    fn get(&self, name: &str) -> Option<i32> {
        self.captures
            .name(name)
            .map(|v| v.as_str().trim().parse().unwrap())
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[cfg(test)]
mod test_day_12 {
    use super::*;

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

        system.apply_steps(1);
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

        system.apply_steps(1);
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
        assert_eq!(system.energy(), 179);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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
        system.apply_steps(10);
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

        assert_eq!(system.energy(), 1940);
    }

    #[test]
    fn calculates_steps_until_repeat() {
        let system = Parser::parse(
            r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
            ",
        );
        assert_eq!(system.steps_until_repeat(), 2772);
    }

    #[test]
    fn calculates_steps_until_repeat_for_system_that_takes_long_time() {
        let system = Parser::parse(
            r"
                <x=-8, y=-10, z=0>
                <x=5, y=5, z=10>
                <x=2, y=-7, z=3>
                <x=9, y=-8, z=-3>
                ",
        );
        assert_eq!(system.steps_until_repeat(), 4_686_774_924);
    }
}
