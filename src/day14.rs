use std::error::Error;
use std::fmt;
use std::fs;
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/14.txt";

#[derive(Debug, PartialEq)]
struct Recipes {
    scores: Vec<u8>,
    elf1_index: usize,
    elf2_index: usize,
}

impl fmt::Display for Recipes {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, score) in self.scores.iter().enumerate() {
            if index == self.elf1_index {
                write!(f, "({})", score)?;
            } else if index == self.elf2_index {
                write!(f, "[{}]", score)?;
            } else {
                write!(f, " {} ", score)?;
            }
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Recipes {
    fn new() -> Recipes {
        Recipes {
            scores: vec![3, 7],
            elf1_index: 0,
            elf2_index: 1,
        }
    }

    fn new_recipes(&mut self) {
        let elf1_score = self.scores[self.elf1_index];
        let elf2_score = self.scores[self.elf2_index];
        let sum = elf1_score + elf2_score;
        fn push_digits(n: u8, digits: &mut Vec<u8>) {
            if n >= 10 {
                push_digits(n / 10, digits);
            }
            digits.push(n % 10);
        }
        push_digits(sum, &mut self.scores);
    }

    fn pick_recipes(&mut self) {
        let elf1_score = self.scores[self.elf1_index];
        let elf2_score = self.scores[self.elf2_index];
        self.elf1_index = (self.elf1_index + (1 + elf1_score as usize)) % self.scores.len();
        self.elf2_index = (self.elf2_index + (1 + elf2_score as usize)) % self.scores.len();
    }

    fn scores_after_n_recipes(&mut self, n: usize, num_scores: usize) -> String {
        while self.scores.len() < n + num_scores {
            self.new_recipes();
            self.pick_recipes();
        }
        self.scores[n..n + num_scores].iter().map(|score| format!("{}", score)).collect()
    }

    fn find_index_of_sequence(&mut self, seq: &[u8]) -> usize {
        let seq_len = seq.len();
        loop {
            let scores_len = self.scores.len();
            self.new_recipes();
            self.pick_recipes();
            let new_scores_len = self.scores.len();
            let diff = new_scores_len - scores_len;
            if scores_len >= seq_len {
                for i in 0..diff {
                    let seq_index = scores_len + i - seq_len;
                    if &self.scores[seq_index..(seq_index + seq_len)] == seq {
                        return seq_index;
                    }
                }
            }
        }
    }
}

fn digit_seq(n: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    for digit in n.chars() {
        digits.push(digit.to_digit(10).unwrap() as u8);
    }
    digits
}

fn read_input_file(filename: &str) -> Result<usize> {
    let input = fs::read_to_string(filename)?;
    Ok(input.trim().parse()?)
}

pub fn solve_part1() -> Result<String> {
    let input = read_input_file(INPUT)?;
    let mut recipes = Recipes::new();
    Ok(recipes.scores_after_n_recipes(input, 10))
}

pub fn solve_part2() -> Result<usize> {
    let input = fs::read_to_string(INPUT)?;
    let mut recipes = Recipes::new();
    let seq = digit_seq(&input.trim());
    Ok(recipes.find_index_of_sequence(&seq[..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_recipes_struct() {
        let recipes = Recipes::new();
        assert_eq!(recipes.scores, vec![3, 7]);
        assert_eq!(recipes.elf1_index, 0);
        assert_eq!(recipes.elf2_index, 1);
    }

    #[test]
    fn adds_new_recipes() {
        let mut recipes = Recipes::new();
        recipes.new_recipes();
        assert_eq!(recipes.scores, vec![3, 7, 1, 0]);
    }

    #[test]
    fn picks_new_recipes() {
        let mut recipes = Recipes::new();
        recipes.new_recipes();
        recipes.pick_recipes();
        assert_eq!(recipes.elf1_index, 0);
        assert_eq!(recipes.elf2_index, 1);
    }

    #[test]
    fn iterates_15_times() {
        let mut recipes = Recipes::new();
        for _ in 0..15 {
            recipes.new_recipes();
            recipes.pick_recipes();
        }
        assert_eq!(
            format!("{}", recipes),
            " 3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2 \n",
        );
    }

    #[test]
    fn scores_after_5_recipes() {
        let mut recipes = Recipes::new();
        assert_eq!(
            recipes.scores_after_n_recipes(5, 10),
            "0124515891",
        );
    }

    #[test]
    fn scores_after_9_recipes() {
        let mut recipes = Recipes::new();
        assert_eq!(
            recipes.scores_after_n_recipes(9, 10),
            "5158916779",
        );
    }

    #[test]
    fn scores_after_18_recipes() {
        let mut recipes = Recipes::new();
        assert_eq!(
            recipes.scores_after_n_recipes(18, 10),
            "9251071085",
        );
    }

    #[test]
    fn scores_after_2018_recipes() {
        let mut recipes = Recipes::new();
        assert_eq!(
            recipes.scores_after_n_recipes(2018, 10),
            "5941429882",
        );
    }

    #[test]
    fn finds_index_of_sequence_1() {
        let mut recipes = Recipes::new();
        let seq = vec![5, 1, 5, 8, 9];
        assert_eq!(
            recipes.find_index_of_sequence(&seq[..]),
            9,
        );
    }

    #[test]
    fn finds_index_of_sequence_2() {
        let mut recipes = Recipes::new();
        let seq = vec![0, 1, 2, 4, 5];
        assert_eq!(
            recipes.find_index_of_sequence(&seq[..]),
            5,
        );
    }

    #[test]
    fn finds_index_of_sequence_3() {
        let mut recipes = Recipes::new();
        let seq = vec![9, 2, 5, 1, 0];
        assert_eq!(
            recipes.find_index_of_sequence(&seq[..]),
            18,
        );
    }

    #[test]
    fn finds_index_of_sequence_4() {
        let mut recipes = Recipes::new();
        let seq = vec![5, 9, 4, 1, 4];
        assert_eq!(
            recipes.find_index_of_sequence(&seq[..]),
            2018,
        );
    }

    #[test]
    fn gets_digit_seq_1() {
        assert_eq!(
            digit_seq("51589"),
            vec![5, 1, 5, 8, 9],
        );
    }

    #[test]
    fn gets_digit_seq_2() {
        assert_eq!(
            digit_seq("01245"),
            vec![0, 1, 2, 4, 5],
        );
    }
}
