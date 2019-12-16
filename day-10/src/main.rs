mod point;

use point::Point;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input();
    challenge_1(&input);
}

fn read_input() -> String {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}

fn challenge_1(input: &str) {
    let max_visible = num_visible_at_best_location(&input);
    println!(
        "Challenge 1: Num visible at best location = {}",
        max_visible
    );
}

fn num_visible_at_best_location(map: &str) -> u32 {
    let locations = extract_locations(map);
    find_max_visible(&locations)
}

fn extract_locations(map: &str) -> Vec<Point> {
    let map: Vec<Vec<char>> = map
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();
    let mut locations: Vec<Point> = Default::default();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                locations.push(Point::new(x as i32, y as i32));
            }
        }
    }
    locations
}

fn find_max_visible(locations: &[Point]) -> u32 {
    let mut max_visible = 0;
    for loc in locations {
        let visible = count_visible_from_loc(locations, *loc);
        if visible > max_visible {
            max_visible = visible;
        }
    }
    max_visible
}

fn count_visible_from_loc(locations: &[Point], loc: Point) -> u32 {
    let mut visible = 0;
    'outer: for other in locations {
        if *other == loc {
            continue;
        }
        let (direction, multiples) = loc.get_to(*other);
        for n in 1..multiples {
            let loc_between = loc + direction * n;
            if locations.contains(&loc_between) {
                continue 'outer;
            }
        }
        visible += 1;
    }
    visible
}

#[allow(dead_code)]
fn satellites_by_direction(locations: &[Point], origin: Point) -> HashMap<Point, Vec<Point>> {
    let mut out: HashMap<Point, Vec<Point>> = Default::default();
    for loc in locations {
        let loc = *loc;
        if loc == origin {
            continue;
        }
        let (direction, _) = origin.get_to(loc);
        out.entry(direction).or_default().push(loc);
    }
    for satellites in out.values_mut() {
        satellites.sort_by_key(|p| origin.squared_distance_to(*p));
    }
    out
}

#[allow(dead_code)]
fn predict_vaporizations(locations: &[Point], origin: Point) -> Vec<Point> {
    let mut by_direction = satellites_by_direction(locations, origin);
    let mut directions: Vec<Point> = by_direction.keys().copied().collect();
    directions.sort_unstable_by(|a, b| a.angle().partial_cmp(&b.angle()).unwrap());
    let mut directions: VecDeque<_> = directions.into();
    let mut out: Vec<Point> = Default::default();
    while !directions.is_empty() {
        let dir = directions.pop_front().unwrap();
        let sats = by_direction.get_mut(&dir).unwrap();
        out.push(sats.remove(0));
        if !sats.is_empty() {
            directions.push_back(dir);
        }
    }
    out
}

#[cfg(test)]
mod day_10_tests {
    use super::*;

    #[test]
    fn finds_num_visible_at_best_location_for_single_asteroid() {
        assert_eq!(num_visible_at_best_location("#"), 0);
    }

    #[test]
    fn finds_num_visible_at_best_location_for_two_asteroids() {
        assert_eq!(num_visible_at_best_location("##"), 1);
    }

    #[test]
    fn finds_num_visible_at_best_location_for_examples() {
        let map = r#"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"#;
        assert_eq!(num_visible_at_best_location(map), 33);

        let map = r#"
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"#;
        assert_eq!(num_visible_at_best_location(map), 35);

        let map = r#"
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
"#;
        assert_eq!(num_visible_at_best_location(map), 41);

        let map = r#"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#;
        assert_eq!(num_visible_at_best_location(map), 210);
    }

    fn build_dir_to_point_map(data: &[((i32, i32), &[(i32, i32)])]) -> HashMap<Point, Vec<Point>> {
        data.into_iter()
            .map(|(k, v)| {
                (
                    Point::new(k.0, k.1),
                    v.iter().map(|p| Point::new(p.0, p.1)).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn generates_satellites_by_direction() {
        let locations = extract_locations("##");

        let origin = Point::new(0, 0);
        let expected = build_dir_to_point_map(&[((1, 0), &[(1, 0)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);

        let origin = Point::new(1, 0);
        let expected = build_dir_to_point_map(&[((-1, 0), &[(0, 0)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);

        let locations = extract_locations("###");
        let origin = Point::new(0, 0);
        let expected = build_dir_to_point_map(&[((1, 0), &[(1, 0), (2, 0)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);

        let locations = extract_locations("#\n#");

        let origin = Point::new(0, 0);
        let expected = build_dir_to_point_map(&[((0, 1), &[(0, 1)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);

        let origin = Point::new(0, 1);
        let expected = build_dir_to_point_map(&[((0, -1), &[(0, 0)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);

        let locations = extract_locations(
            r#"
.#.#
##.#
..##
...#
"#,
        );
        let origin = Point::new(1, 1);
        let expected = build_dir_to_point_map(&[
            ((0, -1), &[(1, 0)]),
            ((2, -1), &[(3, 0)]),
            ((-1, 0), &[(0, 1)]),
            ((1, 0), &[(3, 1)]),
            ((1, 1), &[(2, 2), (3, 3)]),
            ((2, 1), &[(3, 2)]),
        ]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);
    }

    #[test]
    fn satellites_by_direction_returns_closest_satellites_first() {
        let locations = extract_locations("#\n#\n#");
        let origin = Point::new(0, 2);
        let expected = build_dir_to_point_map(&[((0, -1), &[(0, 1), (0, 0)])]);
        assert_eq!(satellites_by_direction(&locations, origin), expected);
    }

    #[test]
    fn calculates_order_of_vaporization() {
        // expected order:
        // .#....###24...#..
        // ##...##.13#67..9#
        // ##...#...5.8####.
        // ..#.....X...###..
        // ..#.#.....#....##
        let locations = extract_locations(
            r#"
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##
"#,
        );
        let origin = Point::new(8, 3);
        let vaporizations = predict_vaporizations(&locations, origin);
        assert_eq!(
            vaporizations[..9],
            [
                Point::new(8, 1),
                Point::new(9, 0),
                Point::new(9, 1),
                Point::new(10, 0),
                Point::new(9, 2),
                Point::new(11, 1),
                Point::new(12, 1),
                Point::new(11, 2),
                Point::new(15, 1),
            ]
        );

        let locations = extract_locations(
            r#"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#,
        );
        let origin = Point::new(11, 13);
        let vaporizations = predict_vaporizations(&locations, origin);
        assert_eq!(vaporizations[0], Point::new(11, 12));
        assert_eq!(vaporizations[1], Point::new(12, 1));
        assert_eq!(vaporizations[2], Point::new(12, 2));
        assert_eq!(vaporizations[9], Point::new(12, 8));
        assert_eq!(vaporizations[19], Point::new(16, 0));
        assert_eq!(vaporizations[49], Point::new(16, 9));
        assert_eq!(vaporizations[99], Point::new(10, 16));
        assert_eq!(vaporizations[198], Point::new(9, 6));
        assert_eq!(vaporizations[199], Point::new(8, 2));
        assert_eq!(vaporizations[200], Point::new(10, 9));
        assert_eq!(vaporizations[298], Point::new(11, 1));
    }
}
