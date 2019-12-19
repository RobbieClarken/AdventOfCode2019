use regex::Regex;
use std::fmt;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    challenge_1(&input);
    challenge_2(&input);
}

fn challenge_1(input: &str) {
    let ore = required_ore(input);
    println!("Challenge 1: Required ore = {}", ore);
}

fn challenge_2(input: &str) {
    let reactions = Reactions::parse(input);
    let fuel = reactions.max_fuel_produced(1_000_000_000_000);
    println!("Challenge 2: Max fuel produced = {}", fuel);
}

fn required_ore(input: &str) -> u64 {
    let reactions = Reactions::parse(input);
    let (ore, _) = reactions.required_ore(Ingredient::new("FUEL", 1), &[]);
    ore
}

struct Reactions(Vec<Reaction>);

impl Reactions {
    fn parse(input: &str) -> Self {
        let mut reactions = Vec::new();
        let re_main =
            Regex::new(r"(?P<inputs>.*) => (?P<qty_out>\d+) (?P<name_out>[A-Z]+)").unwrap();
        let re_inputs = Regex::new(r"(?P<qty>\d+) (?P<name>[A-Z]+)").unwrap();
        for line in input.trim().lines() {
            let captures = re_main.captures(line).unwrap();
            let inputs_str = captures.name("inputs").unwrap().as_str();
            let mut inputs = Vec::new();
            for cap in re_inputs.captures_iter(&inputs_str) {
                let qty = cap.name("qty").unwrap().as_str().parse().unwrap();
                let name = cap.name("name").unwrap().as_str();
                inputs.push(Ingredient::new(name, qty));
            }
            let qty_out = captures.name("qty_out").unwrap().as_str().parse().unwrap();
            let name_out = captures.name("name_out").unwrap().as_str();
            reactions.push(Reaction {
                inputs,
                output: Ingredient::new(name_out, qty_out),
            });
        }
        Self(reactions)
    }

    fn required_ore(
        &self,
        mut output: Ingredient,
        leftovers: &[Ingredient],
    ) -> (u64, Vec<Ingredient>) {
        let mut leftovers = leftovers.to_vec();
        for leftover in leftovers.iter_mut() {
            if leftover.name == output.name {
                let consumed = std::cmp::min(output.quantity, leftover.quantity);
                output.quantity -= consumed;
                leftover.quantity -= consumed;
                break;
            }
        }
        leftovers = leftovers
            .iter()
            .filter(|l| l.quantity > 0)
            .cloned()
            .collect();
        if output.quantity == 0 {
            return (0, leftovers);
        }
        for reaction in &self.0 {
            if reaction.output.name == output.name {
                let mut ore = 0;
                let factor = (output.quantity - 1) / reaction.output.quantity + 1;
                let output_produced = reaction.output.quantity * factor;
                for input in &reaction.inputs {
                    if input.name == "ORE" {
                        ore += input.quantity * factor;
                    } else {
                        let (extra_ore, new_leftovers) = self.required_ore(
                            Ingredient::new(&input.name, input.quantity * factor),
                            &leftovers,
                        );
                        ore += extra_ore;
                        leftovers = new_leftovers;
                    }
                }
                let leftover = output_produced - output.quantity;
                if leftover > 0 {
                    leftovers.push(Ingredient::new(&output.name, leftover));
                }
                return (ore, leftovers);
            }
        }
        unreachable!("Could not find a reaction with output {}", &output);
    }

    fn max_fuel_produced(&self, ore: u64) -> u64 {
        let empty = Vec::new();
        let (ore_for_one_fuel, _) = self.required_ore(Ingredient::new("FUEL", 1), &empty);
        let mut lower_bound = ore / ore_for_one_fuel;
        let mut upper_bound = 2 * lower_bound;
        loop {
            if lower_bound == upper_bound {
                return lower_bound;
            }
            let guess = lower_bound + (upper_bound - lower_bound) / 2;
            let (below, _) = self.required_ore(Ingredient::new("FUEL", guess), &empty);
            if below > ore {
                upper_bound = guess;
                continue;
            }
            let (above, _) = self.required_ore(Ingredient::new("FUEL", guess + 1), &empty);
            if above <= ore {
                lower_bound = guess;
                continue;
            }
            return guess;
        }
    }
}

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    quantity: u64,
}

impl Ingredient {
    fn new(name: &str, quantity: u64) -> Self {
        Self {
            name: name.to_owned(),
            quantity,
        }
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.name)
    }
}

struct Reaction {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.inputs.iter();
        write!(f, "{}", iter.next().unwrap())?;
        for input in iter {
            write!(f, ", {}", input)?;
        }
        write!(f, " => {}", self.output)
    }
}

#[cfg(test)]
mod test_day_14 {
    use super::*;

    #[test]
    fn finds_required_ore_for_simple_case() {
        let input = "
            2 ORE => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 2);
    }

    #[test]
    fn finds_required_ore_for_case_with_simple_dependencies() {
        let input = "
            3 ORE => 1 AAA
            2 AAA => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 6);

        let input = "
            4 ORE => 1 AAA
            3 AAA => 1 BBB
            2 BBB => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 24);

        let input = "
            3 ORE => 2 AAA
            2 AAA => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 3);

        let input = "
            4 ORE => 1 AAA
            3 AAA => 2 BBB
            2 BBB => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 12);

        let input = "
            1 ORE => 2 A
            3 A => 1 B
            2 B => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 3);
    }

    #[test]
    fn handles_multiple_inputs() {
        let input = "
            2 ORE => 1 AAA
            3 ORE, 1 AAA => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 5);

        let input = "
            5 ORE => 2 AAA
            3 ORE, 3 AAA => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 13);
    }

    #[test]
    fn handles_leftovers() {
        let input = "
            1 ORE => 2 AAA
            1 AAA => 1 BBB
            1 AAA, 1 BBB => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 1);
    }

    #[test]
    fn calculates_ore_for_examples() {
        let input = "
            10 ORE => 10 A
            1 ORE => 1 B
            7 A, 1 B => 1 C
            7 A, 1 C => 1 D
            7 A, 1 D => 1 E
            7 A, 1 E => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 31);

        let input = "
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ";
        assert_eq!(required_ore(&input), 165);

        let input = "
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ";
        assert_eq!(required_ore(&input), 13312);

        let input = "
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        ";
        assert_eq!(required_ore(&input), 180697);

        let input = "
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        ";
        assert_eq!(required_ore(&input), 2210736);
    }

    #[test]
    fn calculates_amount_of_ore_needed_to_make_large_amounts_of_fuel() {
        let input = "
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        ";
        let reactions = Reactions::parse(&input);
        let (ore_below, _) = reactions.required_ore(Ingredient::new("FUEL", 460_664), &[]);
        let (ore_above, _) = reactions.required_ore(Ingredient::new("FUEL", 460_665), &[]);
        assert!(ore_below <= 1_000_000_000_000 && ore_above > 1_000_000_000_000);
    }

    #[test]
    fn finds_amount_of_fuel_that_an_be_produced() {
        let ore = 1_000_000_000_000;

        let input = "
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ";
        assert_eq!(Reactions::parse(&input).max_fuel_produced(ore), 82_892_753);

        let input = "
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        ";
        assert_eq!(Reactions::parse(&input).max_fuel_produced(ore), 5_586_022);

        let input = "
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        ";
        assert_eq!(Reactions::parse(&input).max_fuel_produced(ore), 460_664);
    }
}
