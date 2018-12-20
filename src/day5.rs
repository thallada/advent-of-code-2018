extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

const INPUT: &str = "inputs/5.txt";

pub fn solve_part1() -> Result<usize, Box<Error>> {
    let polymer = read_polymer(INPUT)?;
    Ok(reduce_polymer_completely(polymer).len())
}

fn read_polymer(filename: &str) -> Result<String, Box<Error>> {
    let file = File::open(filename)?;
    let polymer = BufReader::new(file).lines().next().unwrap_or(Ok("".to_string()));
    Ok(polymer?)
}

fn reduce_polymer(polymer: &String) -> String {
    lazy_static! {
        static ref REACTING_UNITS: Regex = Regex::new(concat!(
            r"aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|",
            r"oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ|",
            r"Aa|Bb|Cc|Dd|Ee|Ff|Gg|Hh|Ii|Jj|Kk|Ll|Mm|Nn|",
            r"Oo|Pp|Qq|Rr|Ss|Tt|Uu|Vv|Ww|Xx|Yy|Zz")).unwrap();
    }
    REACTING_UNITS.replace_all(&polymer, "").to_string()
}

fn reduce_polymer_completely(polymer: String) -> String {
    let reduced = reduce_polymer(&polymer);
    if reduced == polymer {
        reduced
    } else {
        reduce_polymer_completely(reduced)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/5_test.txt";

    #[test]
    fn reduces_polymer() {
        assert_eq!(reduce_polymer(&"aA".to_string()), "");
        assert_eq!(reduce_polymer(&"aAbB".to_string()), "");
        assert_eq!(reduce_polymer(&"aAfgbB".to_string()), "fg");
        assert_eq!(reduce_polymer(&"abAB".to_string()), "abAB");
        assert_eq!(reduce_polymer(&"aabAAB".to_string()), "aabAAB");
        assert_eq!(reduce_polymer(&"dabAcCaCBAcCcaDA".to_string()), "dabAaCBAcaDA");
        assert_eq!(reduce_polymer(&"dabAaCBAcCcaDA".to_string()), "dabCBAcaDA");
        assert_eq!(reduce_polymer(&"dabCBAcCcaDA".to_string()), "dabCBAcaDA");
    }

    #[test]
    fn reduces_polymer_completely() {
        assert_eq!(reduce_polymer_completely("dabAcCaCBAcCcaDA".to_string()), "dabCBAcaDA");
    }

    #[test]
    fn reads_polymer() {
        assert_eq!(read_polymer(TEST_INPUT).unwrap(), "dabAcCaCBAcCcaDA");
    }
}
