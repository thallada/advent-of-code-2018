extern crate regex;

use std::error::Error;
use std::fmt;
use std::fs;
use std::result;
use std::str::FromStr;

use regex::Regex;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/9_test.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
struct GameParameters {
    players: usize,
    last_marble: usize,
}

impl FromStr for GameParameters {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<GameParameters> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<players>\d+) players; last marble is worth (?P<last_marble>\d+) points"
            )
            .unwrap();
        }

        let captures = match RE.captures(s) {
            None => {
                return Err(From::from(
                    "Malformed game parameters, no fields could be found",
                ));
            }
            Some(captures) => captures,
        };
        Ok(GameParameters {
            players: captures["players"].parse()?,
            last_marble: captures["last_marble"].parse()?,
        })
    }
}

#[derive(Debug, PartialEq)]
struct GameState {
    turn: Option<usize>,
    circle: Vec<usize>,
    current_marble_index: usize,
    current_marble: usize,
    player_scores: Vec<usize>,
}

impl GameState {
    fn new(parameters: GameParameters) -> GameState {
        GameState {
            turn: None,
            circle: vec![0],
            current_marble_index: 0,
            current_marble: 0,
            player_scores: vec![0; parameters.players as usize],
        }
    }

    fn play_until_marble(&mut self, last_marble: usize) {
        println!("{}", &self);
        for _ in 0..last_marble {
            self.turn = match self.turn {
                None => Some(0),
                Some(turn) => Some((turn + 1) % self.player_scores.len()),
            };

            if (self.current_marble + 1) % 23 == 0 {
                self.place_23rd_marble();
            } else {
                self.place_next_marble();
            }
            println!("{}", &self);
        }
        dbg!(&self.player_scores);
    }

    fn place_next_marble(&mut self) {
        self.current_marble += 1;
        if self.current_marble_index == self.circle.len() - 1 {
            self.current_marble_index = 1;
            self.circle.insert(self.current_marble_index, self.current_marble);
        } else {
            self.current_marble_index += 2;
            self.circle.insert(self.current_marble_index, self.current_marble);
        }
    }

    fn place_23rd_marble(&mut self) {
        println!("23rd marble placed");
        self.current_marble += 1;

        // TODO: handle case where this over-extends over the beginning of the vec
        let removed_marble = self.circle.remove(self.current_marble_index - 7);

        self.player_scores[self.turn.unwrap()] += removed_marble + self.current_marble;

        self.current_marble_index -= 7;
    }

    fn highest_score(&mut self) -> usize {
        *self.player_scores.iter().max().unwrap_or(&0)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.turn {
            None => write!(f, "[-] ")?,
            Some(turn) => write!(f, "[{}] ", turn + 1)?,
        }
        for (index, marble) in self.circle.iter().enumerate() {
            if index == self.current_marble_index {
                write!(f, "({}) ", marble)?;
            } else {
                write!(f, "{} ", marble)?;
            }
        }
        Ok(())
    }
}

pub fn solve_part1() -> Result<usize> {
    let game_params = read_game_parameters(INPUT)?;
    Ok(get_highest_score_for_game(game_params))
}

fn read_game_parameters(filename: &str) -> Result<GameParameters> {
    let game_params = fs::read_to_string(filename)?;
    Ok(game_params.parse()?)
}

fn get_highest_score_for_game(game_params: GameParameters) -> usize {
    let mut game_state = GameState::new(game_params);
    game_state.play_until_marble(game_params.last_marble);
    game_state.highest_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/9_test.txt";
    const TEST_GAME_PARAMS: GameParameters = GameParameters {
        players: 9,
        last_marble: 25,
    };

    #[test]
    fn reads_game_parameters_file() {
        assert_eq!(read_game_parameters(TEST_INPUT).unwrap(), TEST_GAME_PARAMS);
    }

    #[test]
    fn gets_highest_score_for_game() {
        assert_eq!(get_highest_score_for_game(TEST_GAME_PARAMS), 32);
    }
}
