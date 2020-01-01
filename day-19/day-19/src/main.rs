use intcode_computer::Computer;

fn main() {
    challenge_1();
    challenge_2();
}

fn challenge_1() {
    let computer = Computer::load_from_file("input");
    println!(
        "Challenge 1: Number of points affected by tractor beam = {}",
        points_in_tractor_beam(computer, (50, 50))
    );
}

fn challenge_2() {
    let computer = Computer::load_from_file("input");
    let mut scan_scale = 100;
    let mut point = None;
    while point.is_none() {
        let scan = scan_area(computer.clone(), (scan_scale, scan_scale));
        point = fit_area((100, 100), scan);
        scan_scale *= 2;
    }
    let point = point.unwrap();
    println!("Challenge 2: Value = {}", 10_000 * point.0 + point.1);
}

fn points_in_tractor_beam(computer: Computer, area: (usize, usize)) -> usize {
    let scan = scan_area(computer, area);
    scan.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |row_acc, v| row_acc + if *v { 1 } else { 0 })
    })
}

fn scan_area(computer: Computer, area: (usize, usize)) -> Vec<Vec<bool>> {
    let mut out = vec![];
    for y in 0..area.1 {
        let mut row = vec![];
        for x in 0..area.0 {
            let mut computer = computer.clone();
            let (output, _) = computer.run(vec![x as i64, y as i64]);
            row.push(output[0] == 1);
        }
        out.push(row);
    }
    out
}

fn fit_area(area: (usize, usize), scan: Vec<Vec<bool>>) -> Option<(usize, usize)> {
    for (y, row) in scan.iter().enumerate() {
        let last_y = y + area.1 - 1;
        if last_y >= scan.len() {
            return None;
        }
        if let Some(x_in_beam_from_right) = row.iter().rev().position(|&v| v) {
            let last_x = row.len() - x_in_beam_from_right - 1;
            if last_x <= area.0 {
                continue;
            }
            let x = last_x - (area.0 - 1);
            if scan[y][x] && scan[last_y][x] {
                return Some((x, y));
            }
        }
    }
    None
}

#[cfg(test)]
mod test_day_19 {
    use super::*;

    #[test]
    fn computes_number_of_points_affected_by_the_tractor_beam() {
        let computer = Computer::load_from_file("../input");
        assert!(points_in_tractor_beam(computer, (10, 10)) > 0);
    }

    #[test]
    fn finds_closest_point_that_fits_area() {
        let scan = "
1000000000000000000000000000000000000000
0100000000000000000000000000000000000000
0011000000000000000000000000000000000000
0001110000000000000000000000000000000000
0000111000000000000000000000000000000000
0000011110000000000000000000000000000000
0000001111100000000000000000000000000000
0000001111110000000000000000000000000000
0000000111111100000000000000000000000000
0000000011111111000000000000000000000000
0000000001111111110000000000000000000000
0000000000111111111000000000000000000000
0000000000011111111110000000000000000000
0000000000011111111111100000000000000000
0000000000001111111111110000000000000000
0000000000000111111111111100000000000000
0000000000000011111111111111000000000000
0000000000000001111111111111110000000000
0000000000000000111111111111111000000000
0000000000000000111111111111111110000000
0000000000000000011111111111111111100000
0000000000000000001111111111111111110000
0000000000000000000111111111111111111100
0000000000000000000011111111111111111111
0000000000000000000001111111111111111111
0000000000000000000001111111111111111111
0000000000000000000000111111111111111111
0000000000000000000000011111111111111111
0000000000000000000000001111111111111111
0000000000000000000000000111111111111111
0000000000000000000000000011111111111111
0000000000000000000000000011111111111111
0000000000000000000000000001111111111111
0000000000000000000000000000111111111111
0000000000000000000000000000011111111111"
            .trim();
        let scan: Vec<Vec<_>> = scan
            .lines()
            .map(|l| l.chars().map(|c| c == '1').collect())
            .collect();
        assert_eq!(fit_area((10, 10), scan), Some((25, 20)));
    }
}
