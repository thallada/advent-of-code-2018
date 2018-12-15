extern crate chrono;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::{HashMap, HashSet};
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

fn read_records(filename: &str) -> Result<Vec<Record>, Box<Error>> {
    let mut records: Vec<Record> = Vec::new();
    let record_regex =
        Regex::new(r"\[(?P<timestamp>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2})\]\s(?:(?P<start>Guard #(?P<guard_id>\d+) begins shift)|(?P<sleep>falls asleep)|(?P<wake>wakes up))")?;
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match record_regex.captures(&line?) {
            Some(captures) => {
                println!("{:?}", NaiveDateTime::parse_from_str(
                    &get_captured_field(&captures, "timestamp")?,
                    "%Y-%m-%d %H:%M")?);
                if has_captured_field(&captures, "start")? {
                    records.push(Record::Start {
                        time: NaiveDateTime::parse_from_str(
                            &get_captured_field(&captures, "timestamp")?,
                            "%Y-%m-%d %H:%M")?,
                        guard_id: get_captured_field(&captures, "guard_id")?.parse()?,
                    });
                } else if has_captured_field(&captures, "sleep")? {
                    records.push(Record::Sleep {
                        time: NaiveDateTime::parse_from_str(
                            &get_captured_field(&captures, "timestamp")?,
                            "%Y-%m-%d %H:%M")?,
                    });
                } else {
                    records.push(Record::Wake {
                        time: NaiveDateTime::parse_from_str(
                            &get_captured_field(&captures, "timestamp")?,
                            "%Y-%m-%d %H:%M")?,
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
}
