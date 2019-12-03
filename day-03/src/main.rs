use std::collections::HashSet;
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
            .nth(0)
            .ok_or_else(|| Error::new(InvalidData, "failed to parse Step direction"))?;
        let distance: u32 = s[1..]
            .parse()
            .map_err(|_| Error::new(InvalidData, "failed to parse Step distance"))?;
        Ok(Self(direction, distance))
    }
}

fn main() -> io::Result<()> {
    let input = read_input("input")?;
    let intersections = find_intersections(&input[0], &input[1]);
    println!("{:?}", intersections);
    Ok(())
}

fn read_input(filename: &str) -> io::Result<Vec<Vec<Step>>> {
    let mut buffer = String::new();
    File::open(filename)?.read_to_string(&mut buffer)?;
    Ok(buffer
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect())
}

fn find_intersections(path1: &[Step], path2: &[Step]) -> Vec<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = Default::default();
    let mut pos = (0, 0);
    for step in path1 {
        let (x, y) = match step.0 {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..step.1 {
            pos = (pos.0 + x, pos.1 + y);
            visited.insert(pos);
        }
    }
    let mut intersections = vec![];
    pos = (0, 0);
    for step in path2 {
        let (x, y) = match step.0 {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unimplemented!(),
        };
        for _ in 0..step.1 {
            pos = (pos.0 + x, pos.1 + y);
            if visited.contains(&pos) {
                intersections.push(pos);
            }
        }
    }
    intersections
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
    fn finds_intersections_when_paths_dont_intersect() {
        assert_eq!(find_intersections(&[Step('L', 1)], &[Step('R', 1)]), vec![]);
    }

    #[test]
    fn finds_intersections_for_trivial_cases() {
        assert_eq!(
            find_intersections(&[Step('L', 1)], &[Step('L', 1)]),
            vec![(-1, 0)]
        );
        assert_eq!(
            find_intersections(&[Step('R', 1)], &[Step('R', 1)]),
            vec![(1, 0)]
        );
        assert_eq!(
            find_intersections(&[Step('U', 1)], &[Step('U', 1)]),
            vec![(0, 1)]
        );
        assert_eq!(
            find_intersections(&[Step('D', 1)], &[Step('D', 1)]),
            vec![(0, -1)]
        );
    }

    #[test]
    fn finds_intersections_for_complex_cases() {
        assert_eq!(
            find_intersections(&[Step('L', 2)], &[Step('L', 2)]),
            vec![(-1, 0), (-2, 0)]
        );
        assert_eq!(
            find_intersections(
                &[Step('L', 1), Step('U', 1), Step('R', 1)],
                &[Step('R', 1), Step('U', 1), Step('L', 1)]
            ),
            vec![(0, 1)]
        );
    }
}
