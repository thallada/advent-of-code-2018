use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "inputs/2.txt";

pub fn solve_part1() -> Result<usize, Box<Error>> {
    calculate_checksum(INPUT)
}

pub fn solve_part2() -> Result<Option<String>, Box<Error>> {
    find_most_common_id_overlap(INPUT)
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

fn find_most_common_id_overlap(filename: &str) -> Result<Option<String>, Box<Error>> {
    let file = File::open(filename)?;
    let mut read_lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_ref = &line?;
        for line_before in read_lines.iter() {
            match ids_are_diff_by_n(line_ref, &line_before, 1) {
                Some(common) => return Ok(Some(common)),
                None => (),
            }
        }

        read_lines.push(line_ref.to_owned());
    }
    Ok(None)
}

fn ids_are_diff_by_n(first: &String, second: &String, n: usize) -> Option<String> {
    if first.len() != second.len() {
        return None;
    }

    let mut diff_count = 0;
    let mut common = String::new();
    let mut first_chars = first.chars();
    let mut second_chars = second.chars();

    for _ in 0..first.len() {
        let first_char = first_chars.next();
        let second_char = second_chars.next();

        if first_char != second_char {
            diff_count += 1
        } else {
            common.push(first_char?);
        }

        if diff_count > n {
            return None;
        }
    }

    Some(common)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = "inputs/2_test.txt";
    const TEST_INPUT_PART_2: &str = "inputs/2_test_part2.txt";

    #[test]
    fn calculates_correct_checksum() {
        assert_eq!(calculate_checksum(TEST_INPUT_PART_1).unwrap(), 12);
    }

    #[test]
    fn ids_are_diff_by_1() {
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("abbdef"), 1).unwrap(),
            "abdef"
        );
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("abcdee"), 1).unwrap(),
            "abcde"
        );
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("bbcdef"), 1).unwrap(),
            "bcdef"
        );
    }

    #[test]
    fn ids_are_diff_by_2() {
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("abbdxf"), 2).unwrap(),
            "abdf"
        );
    }

    #[test]
    fn ids_are_diff_by_n_len_unequal_false() {
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("abbdeff"), 1),
            None
        );
    }

    #[test]
    fn ids_are_diff_by_1_too_many_diffs() {
        assert_eq!(
            ids_are_diff_by_n(&String::from("abcdef"), &String::from("abbdxf"), 1),
            None
        );
    }

    #[test]
    fn finds_most_common_id_overlap() {
        assert_eq!(
            find_most_common_id_overlap(TEST_INPUT_PART_2).unwrap().unwrap(),
            "fgij"
        );
    }
}
