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

fn extract_locations(map: &str) -> Vec<(i32, i32)> {
    let map: Vec<Vec<char>> = map
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();
    let mut locations: Vec<(i32, i32)> = Default::default();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                locations.push((x as i32, y as i32));
            }
        }
    }
    locations
}

fn find_max_visible(locations: &[(i32, i32)]) -> u32 {
    let mut max_visible = 0;
    for loc in locations {
        let visible = count_visible_from_loc(locations, *loc);
        if visible > max_visible {
            max_visible = visible;
        }
    }
    max_visible
}

fn count_visible_from_loc(locations: &[(i32, i32)], loc: (i32, i32)) -> u32 {
    let (x0, y0) = loc;
    let mut visible = 0;
    'outer: for other in locations {
        if *other == loc {
            continue;
        }
        let (x1, y1) = other;
        let x_step = x1 - x0;
        let y_step = y1 - y0;
        let common_divisor = gcd(x_step, y_step);
        if common_divisor == 1 {
            visible += 1;
            continue;
        }
        let x_unit = x_step / common_divisor;
        let y_unit = y_step / common_divisor;
        for n in 1..common_divisor {
            let x_between = x0 + n as i32 * x_unit;
            let y_between = y0 + n as i32 * y_unit;
            if locations.contains(&(x_between, y_between)) {
                continue 'outer;
            }
        }
        visible += 1;
    }
    visible
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x.abs()
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
