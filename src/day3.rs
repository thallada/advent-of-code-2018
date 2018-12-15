extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use regex::{Regex, Captures};

const INPUT: &str = "inputs/3.txt";

#[derive(Debug, PartialEq, Clone)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct MalformedClaim {
    details: String
}

impl MalformedClaim {
    fn new(msg: &str) -> MalformedClaim {
        MalformedClaim{ details: msg.to_string() }
    }
}

impl fmt::Display for MalformedClaim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MalformedClaim {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn solve_part1() -> Result<u32, Box<Error>> {
    Ok(count_overlapping_claimed_points(read_claims(INPUT)?))
}

pub fn solve_part2() -> Result<Option<u32>, Box<Error>> {
    Ok(find_non_overlapping_claim(read_claims(INPUT)?))
}

fn count_overlapping_claimed_points(claims: Vec<Claim>) -> u32 {
    let claimed_points = get_claimed_points(&claims);
    return claimed_points.values().fold(0, |acc, claims| {
        if claims > &1 {
            return acc + 1
        }
        acc
    })
}

fn find_non_overlapping_claim(claims: Vec<Claim>) -> Option<u32> {
    let claimed_points = get_claimed_points(&claims);
    for claim in claims {
        let points = list_points_in_claim(&claim);
        let non_overlapping = points.iter().all(
            |point| claimed_points.get(&point).unwrap_or(&0) < &2);
        if non_overlapping {
            return Some(claim.id);
        }
    }
    None
}

fn get_claimed_points(claims: &Vec<Claim>) -> HashMap<Point, u32> {
    let mut claimed_points: HashMap<Point, u32> = HashMap::new();
    for claim in claims {
        for point in list_points_in_claim(&claim) {
            let current_point = claimed_points.get(&point).unwrap_or(&0);
            claimed_points.insert(point, current_point + 1);
        }
    }
    claimed_points
}

fn list_points_in_claim(claim: &Claim) -> Vec<Point> {
    let mut points = Vec::new();
    for x in 0..claim.width {
        for y in 0..claim.height {
            points.push(Point { x: claim.left + x, y: claim.top + y });
        }
    }
    points
}

fn read_claims(filename: &str) -> Result<Vec<Claim>, Box<Error>> {
    let mut claims: Vec<Claim> = Vec::new();
    let claim_regex =
        Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")?;
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match claim_regex.captures(&line?) {
            Some(captures) => {
                let claim = Claim {
                    id: get_captured_field(&captures, "id")?,
                    left: get_captured_field(&captures, "left")?,
                    top: get_captured_field(&captures, "top")?,
                    width: get_captured_field(&captures, "width")?,
                    height: get_captured_field(&captures, "height")?,
                };
                claims.push(claim);
            },
            None => return Err(Box::new(MalformedClaim {
                details: "Malformed claim line, no fields could be found".to_string()
            })),
        };
    }
    Ok(claims)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<u32, Box<Error>> {
    match captures.name(field) {
        Some(capture) => Ok(capture.as_str().parse()?),
        None => return Err(Box::new(MalformedClaim {
            details: format!("Malformed claim line, field {} could not be found", field)
        }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/3_test.txt";
    const TEST_INPUT_MALFORMED: &str = "inputs/3_test_malformed.txt";

    #[test]
    fn reads_claims_file() {
        assert_eq!(read_claims(TEST_INPUT).unwrap(), vec![
            Claim { id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim { id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim { id: 3, left: 5, top: 5, width: 2, height: 2 },
        ]);
    }

    #[test]
    fn errors_on_malformed_claims_file() {
        match read_claims(TEST_INPUT_MALFORMED) {
            Ok(_) => assert!(false, "read_claims should have returned an error"),
            Err(err) => assert_eq!(
                (*err).description(),
                "Malformed claim line, no fields could be found".to_string(),
            ),
        }
    }

    #[test]
    fn lists_points_in_claim() {
        assert_eq!(list_points_in_claim(&Claim { id: 1, left: 0, top: 0, width: 2, height: 2 }),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ]
        )
    }

    #[test]
    fn counts_overlapping_claimed_points() {
        let test_claims = vec![
            Claim { id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim { id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim { id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(count_overlapping_claimed_points(test_claims), 4);
    }

    #[test]
    fn finds_non_overlapping_claim() {
        let test_claims = vec![
            Claim { id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim { id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim { id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(find_non_overlapping_claim(test_claims).unwrap(), 3);
    }
}
