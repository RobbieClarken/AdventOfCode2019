use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    challenge_1();
}

fn challenge_1() {
    let input = read_input("input");
    println!(
        "Challenge 1: Total number of orbits = {}",
        number_of_orbits(&input)
    );
}

fn read_input(filename: &str) -> String {
    let mut buffer = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}

fn number_of_orbits(s: &str) -> u32 {
    Tree::parse(s).distances_to_root()
}

#[derive(Default)]
struct Tree {
    label: String,
    children: HashMap<String, Vec<String>>,
}

impl Tree {
    fn parse(input: &str) -> Self {
        let mut num_parents_by_label: HashMap<&str, u32> = Default::default();
        let mut tree: Self = Default::default();
        for line in input.trim().lines() {
            let parts: Vec<_> = line.trim().split(')').collect();
            let parent = parts[0];
            let child = parts[1];
            num_parents_by_label.entry(parent).or_insert(0);
            *(num_parents_by_label.entry(child).or_insert(0)) += 1;
            tree.children
                .entry(parent.to_owned())
                .and_modify(|v| (*v).push(child.to_owned()))
                .or_insert_with(|| vec![child.to_owned()]);
        }
        for (label, num_parents) in num_parents_by_label {
            if num_parents == 0 {
                tree.label = label.to_owned();
            }
        }
        assert!(!tree.label.is_empty());
        tree
    }

    fn distances_to_root(&self) -> u32 {
        self.push_distances_below(&self.label, 0, 0)
    }

    fn push_distances_below(&self, parent: &str, depth: u32, mut distances: u32) -> u32 {
        if let Some(children) = self.children.get(parent) {
            let distance_to_here = distances;
            for child in children {
                distances += self.push_distances_below(child, depth + 1, distance_to_here + 1);
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn reads_input() {
        let input = read_input("input");
        assert!(input.starts_with("MQD)G37\nMPH)V45"));
    }

    #[test]
    fn calculates_orbits_for_single_case() {
        assert_eq!(number_of_orbits("A)B"), 1);
    }

    #[test]
    fn calculates_orbits_for_2_orbits_1() {
        assert_eq!(number_of_orbits("A)B\nA)C"), 2);
    }

    #[test]
    fn calculates_orbits_for_3_orbits_1() {
        assert_eq!(number_of_orbits("A)B\nA)C\nA)D"), 3);
    }

    #[test]
    fn calculates_orbits_for_C_orbits_B_orbits_A() {
        assert_eq!(number_of_orbits("A)B\nB)C"), 3);
    }

    #[test]
    fn calculates_orbits_when_root_isnt_first() {
        assert_eq!(number_of_orbits("B)C\nA)B"), 3);
    }

    #[test]
    fn calculates_orbits_for_example() {
        let orbits = number_of_orbits(
            r#"
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            "#,
        );
        assert_eq!(orbits, 42);
    }
}
