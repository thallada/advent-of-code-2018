use std::error::Error;
use std::fs;
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/8.txt";

pub fn solve_part1() -> Result<u32> {
    let license = read_license(INPUT)?;
    Ok(sum_metadata(&license, 0, 0).0)
}

pub fn solve_part2() -> Result<u32> {
    let license = read_license(INPUT)?;
    Ok(sum_metadata_with_indices(&license, 0).0)
}

fn read_license(filename: &str) -> Result<Vec<u32>> {
    let license = fs::read_to_string(filename)?;
    let license = license.trim();
    Ok(license.split(' ').map(|num| num.parse().unwrap()).collect())
}

fn sum_metadata(license: &[u32], mut index: usize, mut sum_acc: u32) -> (u32, usize) {
    let num_children = license[index];
    let num_metadata = license[index + 1];
    index += 2;
    if num_children != 0 {
        for _ in 0..num_children {
            let (child_sum_acc, child_index) = sum_metadata(license, index, sum_acc);
            sum_acc = child_sum_acc;
            index = child_index;
        }
    }
    if num_metadata != 0 {
        let sum: u32 = license[index..index + num_metadata as usize].iter().sum();
        index += num_metadata as usize;
        sum_acc += sum;
    }
    (sum_acc, index)
}

fn sum_metadata_with_indices(license: &[u32], mut index: usize) -> (u32, usize) {
    let mut sum: u32 = 0;
    let num_children = license[index];
    let num_metadata = license[index + 1];
    index += 2;
    if num_children != 0 {
        let mut child_sums: Vec<u32> = Vec::new();
        for _ in 0..num_children {
            let (child_sum, child_index) = sum_metadata_with_indices(license, index);
            index = child_index;
            child_sums.push(child_sum)
        }

        if num_metadata != 0 {
            sum = license[index..index + num_metadata as usize]
                .iter()
                .map(|num| *child_sums.get(*num as usize - 1).unwrap_or(&0))
                .sum();
            index += num_metadata as usize;
        }
    } else if num_metadata != 0 {
        sum = license[index..index + num_metadata as usize].iter().sum();
        index += num_metadata as usize;
    }
    (sum, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/8_test.txt";
    fn test_license() -> Vec<u32> {
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    }

    #[test]
    fn reads_license_file() {
        assert_eq!(read_license(TEST_INPUT).unwrap(), test_license());
    }

    #[test]
    fn sums_license_metadata() {
        assert_eq!(sum_metadata(&test_license(), 0, 0).0, 138);
    }

    #[test]
    fn sums_license_metadata_with_indices() {
        assert_eq!(sum_metadata_with_indices(&test_license(), 0).0, 66);
    }
}
