extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::{Captures, Regex};

const INPUT: &str = "inputs/7.txt";

#[derive(Debug, Clone, PartialEq)]
struct MalformedInstruction {
    details: String,
}

impl MalformedInstruction {
    fn new(msg: &str) -> MalformedInstruction {
        MalformedInstruction {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MalformedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MalformedInstruction {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn solve_part1() -> Result<String, Box<Error>> {
    let mut instructions = read_instructions(INPUT)?;
    Ok(get_step_sequence(&mut instructions))
}

fn read_instructions(filename: &str) -> Result<HashMap<String, Vec<String>>, Box<Error>> {
    let mut instructions: HashMap<String, Vec<String>> = HashMap::new();
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
                let step = get_captured_field(&captures, "step")?.parse()?;
                let dependency: String = get_captured_field(&captures, "dependency")?.parse()?;
                instructions
                    .entry(dependency.clone())
                    .or_insert_with(Vec::new);
                let dependencies = instructions.entry(step).or_insert_with(Vec::new);
                dependencies.push(dependency);
            }
            None => {
                return Err(Box::new(MalformedInstruction {
                    details: "Malformed instruction line, no fields could be found".to_string(),
                }))
            }
        };
    }
    Ok(instructions)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<String, Box<Error>> {
    match captures.name(field) {
        Some(capture) => Ok(String::from(capture.as_str())),
        None => Err(Box::new(MalformedInstruction {
            details: format!(
                "Malformed instruction line, field {} could not be found",
                field
            ),
        })),
    }
}

fn get_step_sequence(instructions: &mut HashMap<String, Vec<String>>) -> String {
    let mut sequence: Vec<String> = Vec::new();
    loop {
        let mut available: Vec<String> = instructions
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(step, _)| step.clone())
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
    sequence.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/7_test.txt";

    #[test]
    fn reads_instructions_file() {
        let expected: HashMap<String, Vec<String>> = [
            ("A".to_string(), vec!["C".to_string()]),
            ("F".to_string(), vec!["C".to_string()]),
            ("C".to_string(), vec![]),
            ("B".to_string(), vec!["A".to_string()]),
            ("D".to_string(), vec!["A".to_string()]),
            (
                "E".to_string(),
                vec!["B".to_string(), "D".to_string(), "F".to_string()],
            ),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(read_instructions(TEST_INPUT).unwrap(), expected);
    }

    #[test]
    fn gets_step_sequence() {
        let mut instructions: HashMap<String, Vec<String>> = [
            ("A".to_string(), vec!["C".to_string()]),
            ("F".to_string(), vec!["C".to_string()]),
            ("C".to_string(), vec![]),
            ("B".to_string(), vec!["A".to_string()]),
            ("D".to_string(), vec!["A".to_string()]),
            (
                "E".to_string(),
                vec!["B".to_string(), "D".to_string(), "F".to_string()],
            ),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(get_step_sequence(&mut instructions), "CABDFE");
    }
}
