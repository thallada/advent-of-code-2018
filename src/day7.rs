extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;

use regex::{Captures, Regex};

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/7.txt";

pub fn solve_part1() -> Result<String> {
    let mut instructions = read_instructions(INPUT)?;
    Ok(get_step_sequence(&mut instructions))
}

fn read_instructions(filename: &str) -> Result<HashMap<char, Vec<char>>> {
    let mut instructions: HashMap<char, Vec<char>> = HashMap::new();
    lazy_static! {
        static ref INSTRUCTION_REGEX: Regex = Regex::new(
            r"Step (?P<dependency>\w) must be finished before step (?P<step>\w) can begin."
        )
        .unwrap();
    }
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match INSTRUCTION_REGEX.captures(&line?) {
            Some(captures) => {
                let step = get_captured_field(&captures, "step")?;
                let dependency: char = get_captured_field(&captures, "dependency")?;
                instructions
                    .entry(dependency)
                    .or_insert_with(Vec::new);
                let dependencies = instructions.entry(step).or_insert_with(Vec::new);
                dependencies.push(dependency);
            }
            None => {
                return Err(From::from(
                    "Malformed instruction line, no fields could be found",
                ))
            }
        };
    }
    Ok(instructions)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<char> {
    match captures.name(field) {
        Some(capture) => match capture.as_str().chars().next() {
            Some(letter) => Ok(letter),
            None => Err(From::from(format!(
                "Malformed instruction line, field {} not a char",
                field
            ))),
        },
        None => Err(From::from(format!(
            "Malformed instruction line, field {} could not be found",
            field
        ))),
    }
}

fn get_step_sequence(instructions: &mut HashMap<char, Vec<char>>) -> String {
    let mut sequence = String::new();
    loop {
        let mut available: Vec<char> = instructions
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(step, _)| *step)
            .collect();
        if available.is_empty() {
            break;
        }
        available.sort();
        available.reverse();
        let next = available.pop().unwrap();
        instructions.remove(&next);
        for dependencies in instructions.values_mut() {
            if let Some(index) = dependencies.iter().position(|d| *d == next) {
                dependencies.remove(index);
            }
        }
        sequence.push(next);
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/7_test.txt";

    fn test_instructions() -> HashMap<char, Vec<char>> {
        [
            ('A', vec!['C']),
            ('F', vec!['C']),
            ('C', vec![]),
            ('B', vec!['A']),
            ('D', vec!['A']),
            (
                'E',
                vec!['B', 'D', 'F'],
            ),
        ]
        .iter()
        .cloned()
        .collect()
    }

    #[test]
    fn reads_instructions_file() {
        assert_eq!(read_instructions(TEST_INPUT).unwrap(), test_instructions());
    }

    #[test]
    fn gets_step_sequence() {
        assert_eq!(get_step_sequence(&mut test_instructions()), "CABDFE");
    }
}
