extern crate chrono;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::iter::FromIterator;

use chrono::prelude::*;
use regex::{Regex, Captures};

const INPUT: &str = "inputs/4.txt";

#[derive(Debug, PartialEq)]
enum Record {
    Start {
        time: NaiveDateTime,
        guard_id: u32,
    },
    Sleep {
        time: NaiveDateTime,
    },
    Wake {
        time: NaiveDateTime,
    },
}

impl Record {
    fn time(&self) -> NaiveDateTime {
        match *self {
            Record::Start { time, guard_id: _ } => time,
            Record::Sleep { time } => time,
            Record::Wake { time } => time,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MalformedRecord {
    details: String
}

impl MalformedRecord {
    fn new(msg: &str) -> MalformedRecord {
        MalformedRecord{ details: msg.to_string() }
    }
}

impl fmt::Display for MalformedRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MalformedRecord {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn solve_part1() -> Result<u32, Box<Error>> {
    Ok(get_part1(INPUT)?)
}

fn get_part1(filename: &str) -> Result<u32, Box<Error>> {
    let records = read_records(filename)?;
    let minutes_asleep = minutes_asleep_per_guard(records);
    let sleepiest_guard = minutes_asleep.iter().max_by_key(|&(_, mins)| mins.len()).unwrap();
    let sleepiest_minute = mode(sleepiest_guard.1);
    Ok(sleepiest_guard.0 * sleepiest_minute)
}

fn mode(numbers: &[u32]) -> u32 {
    let mut occurences = HashMap::new();
    for &value in numbers {
        *occurences.entry(value).or_insert(0) += 1;
    }
    occurences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap_or(0)
}

fn minutes_asleep_per_guard(mut records: Vec<Record>) -> HashMap<u32, Vec<u32>> {
    let mut minutes_asleep: HashMap<u32, Vec<u32>> = HashMap::new();
    records.sort_by_key(|r| r.time());
    let mut current_guard = 0;
    let mut fell_asleep = 0;
    for record in records {
        match record {
            Record::Start { time: _, guard_id } => current_guard = guard_id,
            Record::Sleep { time } => fell_asleep = time.minute(),
            Record::Wake { time } => {
                let mut slept_minutes = (fell_asleep..time.minute()).collect();
                match minutes_asleep.entry(current_guard) {
                    Entry::Vacant(e) => { e.insert(slept_minutes); },
                    Entry::Occupied(mut e) => { e.get_mut().append(&mut slept_minutes); },
                }
            }
        }
    }
    minutes_asleep
}

fn read_records(filename: &str) -> Result<Vec<Record>, Box<Error>> {
    let mut records: Vec<Record> = Vec::new();
    let record_regex =
        Regex::new(concat!(
            r"\[(?P<timestamp>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2})\]\s(?:",
            r"(?P<start>Guard #(?P<guard_id>\d+) begins shift)|",
            r"(?P<sleep>falls asleep)|",
            r"(?P<wake>wakes up))"))?;
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match record_regex.captures(&line?) {
            Some(captures) => {
                let time = NaiveDateTime::parse_from_str(
                    &get_captured_field(&captures, "timestamp")?,
                    "%Y-%m-%d %H:%M")?;
                if has_captured_field(&captures, "start")? {
                    records.push(Record::Start {
                        time: time,
                        guard_id: get_captured_field(&captures, "guard_id")?.parse()?,
                    });
                } else if has_captured_field(&captures, "sleep")? {
                    records.push(Record::Sleep {
                        time: time,
                    });
                } else {
                    records.push(Record::Wake {
                        time: time,
                    });
                }
            },
            None => return Err(Box::new(MalformedRecord {
                details: "Malformed record line, no fields could be found".to_string()
            })),
        };
    }
    Ok(records)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<String, Box<Error>> {
    match captures.name(field) {
        Some(capture) => Ok(String::from(capture.as_str())),
        None => return Err(Box::new(MalformedRecord {
            details: format!("Malformed record line, field {} could not be found", field)
        }))
    }
}

fn has_captured_field(captures: &Captures, field: &str) -> Result<bool, Box<Error>> {
    match captures.name(field) {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/4_test.txt";
    const TEST_INPUT_MALFORMED: &str = "inputs/4_test_malformed.txt";

    #[test]
    fn reads_records_file() {
        assert_eq!(read_records(TEST_INPUT).unwrap(), vec![
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:00", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 10,
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:05", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:25", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:30", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:55", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 23:58", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 99,
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-02 00:40", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-02 00:50", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-03 00:05", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 10,
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-03 00:24", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-03 00:29", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-04 00:02", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 99,
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-04 00:36", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-04 00:46", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-05 00:03", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 99,
            },
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-05 00:45", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-05 00:55", "%Y-%m-%d %H:%M").unwrap(),
            },
        ]);
    }

    #[test]
    fn errors_on_malformed_records_file() {
        match read_records(TEST_INPUT_MALFORMED) {
            Ok(_) => assert!(false, "read_records should have returned an error"),
            Err(err) => assert_eq!(
                (*err).description(),
                "Malformed record line, no fields could be found".to_string(),
            ),
        }
    }

    #[test]
    fn gets_minutes_asleep_per_guard() {
        let mut expected: HashMap<u32, Vec<u32>> = HashMap::new();
        expected.insert(10, vec![5, 6, 7, 8, 9]);
        assert_eq!(minutes_asleep_per_guard(vec![
            Record::Sleep {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:05", "%Y-%m-%d %H:%M").unwrap(),
            },
            Record::Start {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:00", "%Y-%m-%d %H:%M").unwrap(),
                guard_id: 10,
            },
            Record::Wake {
                time: NaiveDateTime::parse_from_str(
                          "1518-11-01 00:10", "%Y-%m-%d %H:%M").unwrap(),
            },
        ]), expected);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(get_part1(TEST_INPUT).unwrap(), 240);
    }
}
