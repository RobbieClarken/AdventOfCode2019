mod point;

use point::Point;
use std::fs::File;
use std::io::Read;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let mut map = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut map)
        .unwrap();
    let max_visible = num_visible_at_best_location(&map);
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
}
