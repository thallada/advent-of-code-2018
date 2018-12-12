use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "inputs/1.txt";

pub fn solve() -> Result<i32, Box<Error>> {
    calculate_resulting_frequency(INPUT)
}

fn calculate_resulting_frequency(filename: &str) -> Result<i32, Box<Error>> {
    let mut freq: i32 = 0;
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        let adjustment: i32 = line?.parse()?;
        freq += adjustment;
    }
    Ok(freq)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/1_test.txt";

    #[test]
    fn finds_resulting_frequency() {
        assert_eq!(calculate_resulting_frequency(TEST_INPUT).unwrap(), 3);
    }
}
