fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let steps = parse(&input);
    challenge_1(&steps);
    challenge_2(&steps);
}

fn challenge_1(steps: &[Shuffle]) {
    let target_card = 2019;
    let cards_in_deck = 10007;
    let mut deck: Vec<u64> = (0..cards_in_deck).collect();
    deck = shuffle_deck(&steps, &deck);
    let position = deck.iter().position(|&v| v == target_card).unwrap();
    println!(
        "Challenge 1: The position of card {} = {}",
        target_card, position
    );
}

fn challenge_2(steps: &[Shuffle]) {
    let target_position = 2020;
    let cards_in_deck = 119_315_717_514_047;
    let number_of_shuffles = 101_741_582_076_661;
    let transform = Transform::new(&steps, cards_in_deck).repeat(number_of_shuffles);
    let card = transform.apply(target_position);
    println!(
        "Challenge 2: The value of the card at position {} = {}",
        target_position, card
    );
}

fn parse(s: &str) -> Vec<Shuffle> {
    s.trim().lines().map(|l| Shuffle::from(l.trim())).collect()
}

fn shuffle_deck(steps: &[Shuffle], deck: &[u64]) -> Vec<u64> {
    let transform = Transform::new(steps, deck.len() as u64);
    deck.iter().map(|&p| transform.apply(p)).collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Shuffle {
    DealIntoNewStack,
    DealWithIncrement(u64),
    Cut(i64),
}

impl Shuffle {
    pub fn from(instruction: &str) -> Self {
        if instruction == "deal into new stack" {
            Self::DealIntoNewStack
        } else if instruction.starts_with("deal with increment") {
            let increment = instruction.split(' ').last().unwrap().parse().unwrap();
            Self::DealWithIncrement(increment)
        } else if instruction.starts_with("cut") {
            let n = instruction.split(' ').last().unwrap().parse().unwrap();
            Self::Cut(n)
        } else {
            unimplemented!("Shuffle '{}' not handled", instruction);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Transform {
    coeff: u64,
    constant: u64,
    modulus: u64,
}

impl Transform {
    fn new(steps: &[Shuffle], modulus: u64) -> Self {
        let mut coeff = 1;
        let mut constant = 0;
        for step in steps.iter().rev() {
            let (new_coeff, new_constant) = match step {
                Shuffle::DealIntoNewStack => (modulus - coeff, modulus - 1 - constant),
                Shuffle::DealWithIncrement(n) => {
                    let n_inv = invert_mod(*n, modulus);
                    (
                        mul_mod(n_inv, coeff, modulus),
                        mul_mod(n_inv, constant, modulus),
                    )
                }
                Shuffle::Cut(n) => {
                    let n = n.rem_euclid(modulus as i64) as u64;
                    (coeff, (constant + n) % modulus)
                }
            };
            coeff = new_coeff;
            constant = new_constant;
        }
        Self {
            coeff,
            constant,
            modulus,
        }
    }

    fn apply(&self, position: u64) -> u64 {
        (mul_mod(position, self.coeff, self.modulus) + self.constant) % self.modulus
    }

    /// Given our transformation is:
    /// T(x) = a*x + b
    /// Through function composition we see:
    /// T^2(x) = T(T(x)) = a*(a*x + b) + b = a^2*x + b*(1 + a)
    /// T^3(x) = T(T(T(x))) = a^3*x + b*(1 + a + a^2)
    /// T^n(x) = a^n*x + b*(1 + a + ... + a^(n-1))
    ///        = pow_mod(a, n, m)*x + b * geom_series(a, n - 1, m)
    fn repeat(&self, n: u64) -> Self {
        let coeff = pow_mod(self.coeff, n as u64, self.modulus);
        let constant = mul_mod(
            self.constant,
            geom_series_mod(self.coeff, n as u64 - 1, self.modulus),
            self.modulus,
        );
        Self {
            constant,
            coeff,
            modulus: self.modulus,
        }
    }
}

/// Finds the greatest common divisor of a and b and values for x and y that satisify:
/// a*x + b*y = gcd(a, b)
/// Returns: (gcd(a, b), x, y)
fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x0, mut x1, mut y0, mut y1) = (0, 1, 1, 0);
    while a != 0 {
        let q = b / a;
        let a_next = b % a;
        b = a;
        a = a_next;
        let y0_next = y1;
        y1 = y0 - q * y1;
        y0 = y0_next;
        let x0_next = x1;
        x1 = x0 - q * x1;
        x0 = x0_next;
    }
    (b, x0, y0)
}

/// Finds y such that:
/// x*y = 1 mod m
fn invert_mod(x: u64, modulus: u64) -> u64 {
    let (gcd, x_inv, _) = extended_gcd(x as i64, modulus as i64);
    assert_eq!(gcd, 1);
    x_inv.rem_euclid(modulus as i64) as u64
}

/// Calculates:
/// a * b % modulus
/// without risk of integer overflow
fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    match (a, b) {
        (0, _) => 0,
        (_, 0) => 0,
        (1, _) => b,
        (_, 1) => a,
        _ => {
            let a_b_div2 = mul_mod(a, b / 2, modulus);
            if b & 1 == 0 {
                (a_b_div2 + a_b_div2) % modulus
            } else {
                (a + a_b_div2 + a_b_div2) % modulus
            }
        }
    }
}

/// Calculates:
/// base ^ exp % modulus
fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        exp >>= 1;
        base = mul_mod(base, base, modulus);
    }
    result
}

/// Calculates: 1 + base + base^2 + base^3 + ... + base^n % modulus
fn geom_series_mod(base: u64, n: u64, modulus: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let base_squared = mul_mod(base, base, modulus);
    match n & 1 {
        0 => {
            (1 + mul_mod(
                base + base_squared,
                geom_series_mod(base_squared, (n - 2) / 2, modulus),
                modulus,
            )) % modulus
        }
        1 => mul_mod(
            1 + base,
            geom_series_mod(base_squared, (n - 1) / 2, modulus),
            modulus,
        ),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test_day_22 {
    use super::*;

    #[test]
    fn parses_steps() {
        let s = "
            deal into new stack
            deal with increment 123
            cut -456
        ";
        let steps = parse(s);
        assert_eq!(
            steps,
            vec![
                Shuffle::DealIntoNewStack,
                Shuffle::DealWithIncrement(123),
                Shuffle::Cut(-456),
            ]
        );
    }

    #[test]
    fn verify_examples() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let steps = parse(
            "
            deal with increment 7
            deal into new stack
            deal into new stack
            ",
        );
        assert_eq!(
            shuffle_deck(&steps, &deck),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );

        let steps = parse(
            "
            cut 6
            deal with increment 7
            deal into new stack
            ",
        );
        assert_eq!(
            shuffle_deck(&steps, &deck),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        );

        let steps = parse(
            "
            deal with increment 7
            deal with increment 9
            cut -2
            ",
        );
        assert_eq!(
            shuffle_deck(&steps, &deck),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        );

        let steps = parse(
            "
            deal into new stack
            cut -2
            deal with increment 7
            cut 8
            cut -4
            deal with increment 7
            cut 3
            deal with increment 9
            deal with increment 3
            cut -1
            ",
        );
        assert_eq!(
            shuffle_deck(&steps, &deck),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        );
    }

    #[test]
    fn calculates_extended_gcd() {
        let a = 3 * 4;
        let b = 4 * 5;
        let (gcd, x, y) = extended_gcd(a, b);
        assert_eq!(a * x + b * y, gcd);
        assert_eq!(gcd, 4);
    }

    #[test]
    fn calculates_modular_inverse() {
        let x = 3;
        let inv = invert_mod(x, 10);
        assert_eq!(inv, 7);
        assert_eq!((x * inv) % 10, 1);
    }

    #[test]
    fn outputs_shuffle_equation() {
        let steps = parse("deal into new stack");
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 9,
                constant: 9,
                modulus: 10,
            }
        );

        let steps = parse("deal with increment 3");
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 7,
                constant: 0,
                modulus: 10,
            }
        );

        let steps = parse("cut 6");
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 1,
                constant: 6,
                modulus: 10,
            }
        );

        let steps = parse(
            "
            deal into new stack
            deal with increment 3
            ",
        );
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 3,
                constant: 9,
                modulus: 10,
            }
        );

        let steps = parse(
            "
            deal with increment 3
            deal into new stack
            ",
        );
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 3,
                constant: 3,
                modulus: 10,
            }
        );

        let steps = parse(
            "
            cut 5
            deal into new stack
            ",
        );
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 9,
                constant: 4,
                modulus: 10,
            }
        );

        let steps = parse(
            "
            deal into new stack
            cut -2
            deal with increment 7
            cut 8
            cut -4
            deal with increment 7
            cut 3
            deal with increment 9
            deal with increment 3
            cut -1
            ",
        );
        assert_eq!(
            Transform::new(&steps, 10),
            Transform {
                coeff: 3,
                constant: 9,
                modulus: 10,
            }
        );
    }

    #[test]
    fn transform_can_be_repeated() {
        let steps = parse(
            "
            cut 6
            deal with increment 7
            deal into new stack
            ",
        );
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _ in 0..5 {
            deck = shuffle_deck(&steps, &deck);
        }
        assert_eq!(deck, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let transform = Transform::new(&steps, 10).repeat(5);
        assert_eq!(transform.apply(0), 3);
        assert_eq!(transform.apply(1), 0);
        assert_eq!(transform.apply(2), 7);
    }

    #[test]
    fn calculates_pow_with_modulo() {
        assert_eq!(pow_mod(3, 4, 11), 4);
    }

    #[test]
    fn calculates_geometric_series_with_modulo() {
        let expected_result = (1 + 3 + 9 + 27 + 81 + 243) % 10;
        assert_eq!(geom_series_mod(3, 5, 10), expected_result);
    }
}
