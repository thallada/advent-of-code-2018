extern crate regex;

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::result;
use std::str::FromStr;

use regex::Regex;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/12.txt";

#[derive(Debug, PartialEq)]
struct Pots(VecDeque<bool>);

#[derive(Debug, PartialEq)]
struct SpreadRules(HashMap<[bool; 5], bool>);

#[derive(Debug, PartialEq)]
struct GrowthSimulation {
    pots: Pots,
    spread_rules: SpreadRules,
    generations: usize,
}

impl fmt::Display for Pots {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pot in self.0.iter() {
            if *pot {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl fmt::Display for SpreadRules {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pattern, result) in self.0.iter() {
            for pot in pattern {
                if *pot {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, " => ")?;
            if *result {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl fmt::Display for GrowthSimulation {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pots)?;
        write!(f, "\n")?;
        write!(f, "{}", self.spread_rules)?;
        Ok(())
    }
}

impl FromStr for GrowthSimulation {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<GrowthSimulation> {
        let sections: Vec<&str> = s.split("\n\n").collect();
        let (initial_state_str, spread_rules_str) = (sections[0], sections[1]);
        let mut pots = Pots(VecDeque::new());
        let mut spread_rules = SpreadRules(HashMap::new());

        lazy_static! {
            static ref initial_state_regex: Regex =
                Regex::new(r"initial state: (?P<initial_state>[#.]+)").unwrap();
            static ref spread_rules_regex: Regex =
                Regex::new(r"(?P<pattern>[#.]{5}) => (?P<result>[#.])").unwrap();
        }

        let initial_state_captures = match initial_state_regex.captures(initial_state_str) {
            None => {
                return Err(From::from(
                    "Malformed initial state, no fields could be found",
                ));
            }
            Some(captures) => captures,
        };

        for pot in initial_state_captures["initial_state"].chars() {
            pots.0.push_back(pot == '#');
        }

        for rule in spread_rules_str.lines() {
            let spread_rules_captures = match spread_rules_regex.captures(rule) {
                None => {
                    return Err(From::from(
                        "Malformed spread rules, no fields could be found",
                    ));
                }
                Some(captures) => captures,
            };

            let mut pattern = [false; 5];
            let pattern_vec: Vec<bool> = spread_rules_captures["pattern"]
                .chars()
                .map(|c| c == '#')
                .collect();
            pattern.copy_from_slice(&pattern_vec);
            let result = &spread_rules_captures["result"] == "#";
            spread_rules.0.insert(pattern, result);
        }

        Ok(GrowthSimulation { pots, spread_rules, generations: 0 })
    }
}

impl GrowthSimulation {
    fn advance_generation(&mut self) {
        let mut next_generation = VecDeque::new();
        let padding = &[false; 4];
        let pots_slice = self.pots.0.as_slices();
        let pots_slice = [padding, pots_slice.0, pots_slice.1, padding].concat();
        for (index, pot_window) in pots_slice.windows(5).enumerate() {
            match self.spread_rules.0.get(pot_window) {
                Some(result) => {
                    next_generation.push_back(*result);
                },
                None => {
                    next_generation.push_back(pots_slice[2]);
                },
            }
        }
        self.pots = Pots(next_generation);
        self.generations += 1;
    }

    fn sum_plant_indices(&self) -> i32 {
        let mut sum: i32 = 0;
        for (index, pot) in self.pots.0.iter().enumerate() {
            if *pot {
                let shifted_index = index as i32 - self.generations as i32 * 2;
                sum += shifted_index;
            }
        }
        sum
    }
}

fn read_initial_state_and_rules(filename: &str) -> Result<GrowthSimulation> {
    let input = fs::read_to_string(filename)?;
    Ok(input.parse()?)
}

pub fn solve_part1() -> Result<i32> {
    let mut growth_sim = read_initial_state_and_rules(INPUT)?;
    for _ in 0..20 {
        growth_sim.advance_generation();
    }
    Ok(growth_sim.sum_plant_indices())
}

pub fn solve_part2() -> Result<i32> {
    let mut growth_sim = read_initial_state_and_rules(INPUT)?;
    for _ in 0..50_000_000_000_i64 {
        growth_sim.advance_generation();
    }
    Ok(growth_sim.sum_plant_indices())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/12_test.txt";
    fn test_growth_sim() -> GrowthSimulation {
        GrowthSimulation {
            pots: Pots(VecDeque::from(vec![
                true, false, false, true, false, true, false, false, true, true, false, false,
                false, false, false, false, true, true, true, false, false, false, true, true,
                true,
            ])),
            spread_rules: SpreadRules([
                ([true, true, true, true, false], true),
                ([true, true, false, true, true], true),
                ([false, false, true, false, false], true),
                ([false, true, false, false, false], true),
                ([true, true, false, true, false], true),
                ([false, true, true, true, true], true),
                ([true, false, true, true, true], true),
                ([true, true, true, false, true], true),
                ([false, true, false, true, true], true),
                ([true, false, true, false, true], true),
                ([false, true, false, true, false], true),
                ([false, true, true, false, false], true),
                ([true, true, true, false, false], true),
                ([false, false, false, true, true], true),
            ]
            .iter()
            .cloned()
            .collect()),
            generations: 0,
        }
    }

    #[test]
    fn reads_initial_state_and_rules_file() {
        let growth_sim = read_initial_state_and_rules(TEST_INPUT).unwrap();
        assert_eq!(growth_sim, test_growth_sim());
    }

    #[test]
    fn displays_growth_simulation() {
        assert_eq!(
            format!("{}", test_growth_sim()).lines().collect::<Vec<&str>>().sort(),
            vec![
                "#..#.#..##......###...###",
                "",
                "####. => #",
                "##.## => #",
                "..#.. => #",
                ".#... => #",
                "##.#. => #",
                ".#### => #",
                "#.### => #",
                "###.# => #",
                ".#.## => #",
                "#.#.# => #",
                ".#.#. => #",
                ".##.. => #",
                "###.. => #",
                "...## => #",
            ].sort()
        );
    }

    #[test]
    fn advances_simulation_by_one_generation() {
        let mut growth_sim = test_growth_sim();
        growth_sim.advance_generation();
        assert_eq!(format!("{}", growth_sim.pots), "..#...#....#.....#..#..#..#..\n");
    }

    #[test]
    fn returns_correct_sum_after_20_generations() {
        let mut growth_sim = test_growth_sim();
        for _ in 0..20 {
            growth_sim.advance_generation();
        }
        assert_eq!(growth_sim.sum_plant_indices(), 325);
    }
}
