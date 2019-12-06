const TEN: u32 = 10;

fn main() {
    println!("Challenge 1: {}", valid_count(359282, 820401));
}

fn valid_count(start: u32, end: u32) -> u32 {
    let mut count = 0;
    for n in start..=end {
        if check(n) {
            count += 1;
        }
    }
    count
}

fn check(n: u32) -> bool {
    let s = format!("{}", n);
    if !(s.len() == 6) {
        return false;
    }
    let mut last_digit = n / TEN.pow(5);
    let mut found_adj = false;
    for p in (1..=5).rev() {
        let digit = (n % TEN.pow(p)) / TEN.pow(p - 1);
        if digit < last_digit {
            return false;
        }
        found_adj |= digit == last_digit;
        last_digit = digit;
    }
    found_adj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_6_digits() {
        assert!(!check(11345));
        assert!(!check(1134567));

        assert!(check(113456));
    }

    #[test]
    fn requires_two_adjacent_digits() {
        assert!(!check(123456));

        assert!(check(113456));
        assert!(check(122456));
        assert!(check(123356));
        assert!(check(123446));
        assert!(check(123455));
    }

    #[test]
    fn requires_monotonicity() {
        assert!(!check(102345));
        assert!(!check(112134));
        assert!(!check(112340));
    }

    #[test]
    fn verify_examples() {
        assert!(check(111111));
        assert!(!check(223450));
        assert!(!check(123789));
    }

    #[test]
    fn calculates_valid_count() {
        assert_eq!(valid_count(123450, 123470), 2);
    }
}
