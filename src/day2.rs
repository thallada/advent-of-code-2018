use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const INPUT: &str = "inputs/2.txt";

pub fn solve() -> Result<usize, Box<Error>> {
    calculate_checksum(INPUT)
}

fn calculate_checksum(filename: &str) -> Result<usize, Box<Error>> {
    let mut two_count = 0;
    let mut three_count = 0;
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        for c in line?.chars() {
            let current_count = char_map.get(&c).unwrap_or(&0);
            char_map.insert(c, current_count + 1);
        }

        if char_map.values().any(|&count| count == 2) {
            two_count += 1
        }
        if char_map.values().any(|&count| count == 3) {
            three_count += 1
        }
    }
    Ok(two_count * three_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/2_test.txt";

    #[test]
    fn calculates_correct_checksum() {
        assert_eq!(calculate_checksum(TEST_INPUT).unwrap(), 12);
    }
}
