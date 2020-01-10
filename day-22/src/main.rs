const CARDS_IN_DECK: usize = 10007;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = std::fs::read_to_string("input").unwrap();
    let steps = parse(&input);
    let deck: Vec<u32> = (0..(CARDS_IN_DECK as u32)).collect();
    let final_deck = apply(steps, &deck);
    let position = final_deck.iter().position(|&v| v == 2019).unwrap();
    println!("Challenge 1: The position of card 2019 = {}", position);
}

#[derive(Debug, PartialEq, Eq)]
enum Technique {
    DealIntoNewStack,
    DealWithIncrement(usize),
    Cut(isize),
}

use Technique::*;

impl Technique {
    pub fn from(s: &str) -> Self {
        if s == "deal into new stack" {
            return DealIntoNewStack;
        } else if s.starts_with("deal with increment") {
            let increment = s.split(' ').last().unwrap().parse().unwrap();
            return DealWithIncrement(increment);
        } else if s.starts_with("cut") {
            let n = s.split(' ').last().unwrap().parse().unwrap();
            return Cut(n);
        }
        unimplemented!("Technique '{}' not handled", s);
    }

    pub fn execute(&self, deck: &[u32]) -> Vec<u32> {
        match self {
            DealIntoNewStack => self.deal_into_new_stack(deck),
            Cut(n) => self.cut(*n, deck),
            DealWithIncrement(inc) => self.deal_with_increment(*inc, deck),
        }
    }

    fn deal_into_new_stack(&self, deck: &[u32]) -> Vec<u32> {
        let mut new_deck = deck.to_vec();
        new_deck.reverse();
        new_deck
    }

    fn deal_with_increment(&self, increment: usize, deck: &[u32]) -> Vec<u32> {
        let mut new_deck = deck.to_vec();
        let mut i = 0;
        for v in deck {
            new_deck[i] = *v;
            i = (i + increment) % deck.len();
        }
        new_deck
    }

    fn cut(&self, n: isize, deck: &[u32]) -> Vec<u32> {
        let mut new_deck = deck.to_vec();
        if n < 0 {
            new_deck.rotate_right(n.abs() as usize);
        } else {
            new_deck.rotate_left(n as usize);
        }
        new_deck
    }
}

fn parse(s: &str) -> Vec<Technique> {
    s.trim()
        .lines()
        .map(|l| Technique::from(l.trim()))
        .collect()
}

fn apply(steps: Vec<Technique>, deck: &[u32]) -> Vec<u32> {
    steps
        .iter()
        .fold(deck.to_vec(), |deck, step| step.execute(&deck))
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
            vec![DealIntoNewStack, DealWithIncrement(123), Cut(-456),]
        );
    }

    #[test]
    fn executes_deal_into_new_stack() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let out = DealIntoNewStack.execute(&deck);
        assert_eq!(out, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn executes_deal_with_increment() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            DealWithIncrement(3).execute(&deck),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn executes_cut() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Cut(3).execute(&deck), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        assert_eq!(Cut(-4).execute(&deck), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
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
        assert_eq!(apply(steps, &deck), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let steps = parse(
            "
            cut 6
            deal with increment 7
            deal into new stack
        ",
        );
        assert_eq!(apply(steps, &deck), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let steps = parse(
            "
            deal with increment 7
            deal with increment 9
            cut -2
        ",
        );
        assert_eq!(apply(steps, &deck), vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

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
        assert_eq!(apply(steps, &deck), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
