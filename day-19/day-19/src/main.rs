use intcode_computer::Computer;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let computer = Computer::load_from_file("input");
    println!(
        "Challenge 1: Number of points affected by tractor beam = {}",
        points_in_tractor_beam(computer, (50, 50))
    );
}

fn points_in_tractor_beam(computer: Computer, area: (usize, usize)) -> usize {
    let mut num_points = 0;
    for x in 0..area.0 {
        for y in 0..area.1 {
            let mut computer = computer.clone();
            let (output, _) = computer.run(vec![x as i64, y as i64]);
            num_points += output[0] as usize;
        }
    }
    num_points
}

#[cfg(test)]
mod test_day_19 {
    use super::*;

    #[test]
    fn computes_number_of_points_affected_by_the_tractor_beam() {
        let computer = Computer::load_from_file("../input");
        assert!(points_in_tractor_beam(computer, (10, 10)) > 0);
    }
}
