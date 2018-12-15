use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const INPUT: &str = "inputs/1.txt";
const LOOP_LIMIT: u16 = 1000;

pub fn solve_part1() -> Result<i32, Box<Error>> {
    calculate_resulting_frequency(INPUT)
}

pub fn solve_part2() -> Result<Option<i32>, Box<Error>> {
    find_repeating_frequency(INPUT)
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

fn find_repeating_frequency(filename: &str) -> Result<Option<i32>, Box<Error>> {
    let freqs = read_frequencies(filename)?;
    let mut result_freqs = HashSet::new();
    let mut freq: i32 = 0;
    let mut loop_count = 0;
    while loop_count < LOOP_LIMIT {
        for adjustment in &freqs {
            freq += adjustment;
            if result_freqs.contains(&freq) {
                return Ok(Some(freq))
            } else {
                result_freqs.insert(freq);
            }
        }
        loop_count += 1
    }
    Ok(None)
}

fn read_frequencies(filename: &str) -> Result<Vec<i32>, Box<Error>> {
    let mut freqs: Vec<i32> = Vec::new();
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        freqs.push(line?.parse()?);
    }
    Ok(freqs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/1_test.txt";
    const TEST_INPUT_PART_2: &str = "inputs/1_test_part2.txt";
    const TEST_INPUT_PART_2_2: &str = "inputs/1_test_part2_2.txt";
    const TEST_INPUT_PART_2_3: &str = "inputs/1_test_part2_3.txt";

    #[test]
    fn finds_resulting_frequency() {
        assert_eq!(calculate_resulting_frequency(TEST_INPUT).unwrap(), 3);
    }

    #[test]
    fn finds_repeating_frequency() {
        assert_eq!(find_repeating_frequency(TEST_INPUT_PART_2).unwrap().unwrap(), 14);
    }

    #[test]
    fn finds_repeating_frequency_2() {
        assert_eq!(find_repeating_frequency(TEST_INPUT_PART_2_2).unwrap().unwrap(), 10);
    }

    #[test]
    fn finds_repeating_frequency_3() {
        assert_eq!(find_repeating_frequency(TEST_INPUT_PART_2_3).unwrap().unwrap(), 5);
    }

    #[test]
    fn reads_frequencies_file() {
        assert_eq!(read_frequencies(TEST_INPUT).unwrap(), vec![5, -5, 3]);
    }
}
