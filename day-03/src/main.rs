use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Step(char, u32);

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s
            .chars()
            .next()
            .ok_or_else(|| Error::new(InvalidData, "failed to parse Step direction"))?;
        let distance: u32 = s[1..]
            .parse()
            .map_err(|_| Error::new(InvalidData, "failed to parse Step distance"))?;
        Ok(Self(direction, distance))
    }
}

fn main() -> io::Result<()> {
    let input = read_input("input")?;
    if let Some(distance) = min_distance_for_paths(&input[0], &input[1]) {
        println!("Challenge 1: {}", distance);
    } else {
        println!("Challenge 1: No intersections found");
    }
    if let Some(distance) = min_steps_for_paths(&input[0], &input[1]) {
        println!("Challenge 2: {}", distance);
    } else {
        println!("Challenge 2: No intersections found");
    }
    Ok(())
}

fn min_distance_for_paths(path1: &[Step], path2: &[Step]) -> Option<u32> {
    min_distance(&find_intersections(path1, path2))
}

fn min_steps_for_paths(path1: &[Step], path2: &[Step]) -> Option<u32> {
    min_steps(&find_intersections(path1, path2), path1, path2)
}

fn read_input(filename: &str) -> io::Result<Vec<Vec<Step>>> {
    let mut buffer = String::new();
    File::open(filename)?.read_to_string(&mut buffer)?;
    Ok(buffer.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<Step> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn find_intersections(path1: &[Step], path2: &[Step]) -> Vec<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = Default::default();
    walk(path1, |pos| {
        visited.insert(pos);
    });
    let mut intersections = vec![];
    walk(path2, |pos| {
        if visited.contains(&pos) {
            intersections.push(pos);
        }
    });
    intersections
}

fn walk<F>(path: &[Step], mut func: F)
where
    F: FnMut((i32, i32)) -> (),
{
    let mut pos = (0, 0);
    for step in path {
        let (x, y) = match step.0 {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unimplemented!(),
        };
        for _ in 0..step.1 {
            pos = (pos.0 + x, pos.1 + y);
            func(pos);
        }
    }
}

fn manhattan_distance(loc: (i32, i32)) -> u32 {
    (loc.0.abs() + loc.1.abs()) as u32
}

fn min_distance(points: &[(i32, i32)]) -> Option<u32> {
    points.iter().map(|p| manhattan_distance(*p)).min()
}

fn min_steps(points: &[(i32, i32)], path1: &[Step], path2: &[Step]) -> Option<u32> {
    if points.is_empty() {
        return None;
    }
    let mut steps: HashMap<(i32, i32), Vec<u32>> = Default::default();
    let mut path_steps = 0;
    walk(path1, |pos| {
        path_steps += 1;
        if points.contains(&pos) {
            steps.entry(pos).or_insert_with(|| vec![path_steps]);
        }
    });
    path_steps = 0;
    walk(path2, |pos| {
        path_steps += 1;
        if points.contains(&pos) {
            let v = steps.get_mut(&pos).unwrap();
            if v.len() < 2 {
                v.push(path_steps);
            }
        }
    });
    steps
        .iter()
        .map(|(_, path_steps)| path_steps.iter().sum())
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_input() {
        let input = read_input("input").unwrap();
        assert_eq!(input.len(), 2);
        assert_eq!(
            input[0][..3],
            [Step('R', 1000), Step('D', 940), Step('L', 143)]
        );
        assert_eq!(
            input[1][..3],
            [Step('L', 990), Step('D', 248), Step('L', 833)]
        );
    }

    #[test]
    fn finds_min_distance_for_paths() {
        let path1 = parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_line("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(min_distance_for_paths(&path1, &path2), Some(159));

        let path1 = parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(min_distance_for_paths(&path1, &path2), Some(135));
    }

    #[test]
    fn finds_min_steps_for_paths() {
        let path1 = parse_line("R8,U5,L5,D3");
        let path2 = parse_line("U7,R6,D4,L4");
        assert_eq!(min_steps_for_paths(&path1, &path2), Some(30));

        let path1 = parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = parse_line("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(min_steps_for_paths(&path1, &path2), Some(610));

        let path1 = parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(min_steps_for_paths(&path1, &path2), Some(410));
    }
}
