const TEN: u32 = 10;

fn main() {
    println!("Challenge 1: {}", valid_count(359282, 820401, check1));
    println!("Challenge 2: {}", valid_count(359282, 820401, check2));
}

fn valid_count<F>(start: u32, end: u32, check_func: F) -> u32
where
    F: Fn(u32) -> bool,
{
    let mut count = 0;
    for n in start..=end {
        if check_func(n) {
            count += 1;
        }
    }
    count
}

fn check1(n: u32) -> bool {
    if format!("{}", n).len() != 6 {
        return false;
    }
    let mut last_digit = n / TEN.pow(5);
    let mut found_adj = false;
    for p in (0..=4).rev() {
        let digit = (n % TEN.pow(p + 1)) / TEN.pow(p);
        if digit < last_digit {
            return false;
        }
        found_adj |= digit == last_digit;
        last_digit = digit;
    }
    found_adj
}

fn check2(n: u32) -> bool {
    if format!("{}", n).len() != 6 {
        return false;
    }
    let mut last_digit = n / TEN.pow(5);
    let mut consecutive = 1;
    let mut found_two = false;
    for p in (0..=4).rev() {
        let digit = (n % TEN.pow(p + 1)) / TEN.pow(p);
        if digit < last_digit {
            return false;
        }
        if digit == last_digit {
            consecutive += 1;
            continue;
        }
        found_two |= consecutive == 2;
        last_digit = digit;
        consecutive = 1;
    }
    found_two | (consecutive == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1_requires_6_digits() {
        assert!(!check1(11345));
        assert!(!check1(1134567));

        assert!(check1(113456));
    }

    #[test]
    fn check1_requires_two_adjacent_digits() {
        assert!(!check1(123456));

        assert!(check1(113456));
        assert!(check1(122456));
        assert!(check1(123356));
        assert!(check1(123446));
        assert!(check1(123455));
    }

    #[test]
    fn check1_requires_monotonicity() {
        assert!(!check1(102345));
        assert!(!check1(112134));
        assert!(!check1(112340));
    }

    #[test]
    fn verify_examples_1() {
        assert!(check1(111111));
        assert!(!check1(223450));
        assert!(!check1(123789));
    }

    #[test]
    fn check2_requires_6_digits() {
        assert!(!check2(11345));
        assert!(!check2(1134567));

        assert!(check2(113456));
    }

    #[test]
    fn check2_requires_two_adjacent_digits() {
        assert!(!check2(123456));

        assert!(check2(113456));
        assert!(check2(122456));
        assert!(check2(123356));
        assert!(check2(123446));
        assert!(check2(123455));
    }

    #[test]
    fn check2_rejects_when_there_arent_exactly_two_adjacent_digits() {
        assert!(!check2(111234));
        assert!(check2(111223));
        assert!(!check2(111222));
    }

    #[test]
    fn check2_requires_monotonicity() {
        assert!(!check2(102345));
        assert!(!check2(112134));
        assert!(!check2(112340));
    }

    #[test]
    fn verify_examples_2() {
        assert!(check2(112233));
        assert!(!check2(123444));
        assert!(check2(111122));
    }

    #[test]
    fn calculates_valid_count() {
        assert_eq!(valid_count(123450, 123470, check1), 2);
    }
}
