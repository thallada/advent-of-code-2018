use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::result;
use std::str::FromStr;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/13.txt";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vector {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
    Reverse,
}

const INTER_SEQ: [Turn; 3] = [Turn::Left, Turn::Straight, Turn::Right];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cart {
    position: Vector,
    direction: Direction,
    next_turn: u8,
}

#[derive(Debug, PartialEq)]
struct Track {
    carts: Vec<Cart>,
    turns: HashMap<Vector, (Direction, Direction)>,
    intersections: HashSet<Vector>,
}

#[derive(Debug, PartialEq)]
struct Collision {
    position: Vector,
    cart_indices: (usize, usize),
}

impl FromStr for Track {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Track> {
        let mut carts = vec![];
        let mut turns = HashMap::new();
        let mut intersections = HashSet::new();
        let mut incomplete_circuits: HashMap<(usize, usize), usize> = HashMap::new();

        for (row_index, row) in s.lines().enumerate() {
            let mut top_start: Option<usize> = None;
            let mut bottom_start: Option<usize> = None;
            for c in row.char_indices() {
                match c {
                    (col_index, '\\') => match top_start {
                        Some(start) => {
                            incomplete_circuits.insert((start, col_index), row_index);
                            turns.insert(
                                Vector {
                                    x: col_index,
                                    y: row_index,
                                },
                                (Direction::South, Direction::West),
                            );
                            top_start = None;
                        }
                        None => {
                            bottom_start = Some(col_index);
                            turns.insert(
                                Vector {
                                    x: col_index,
                                    y: row_index,
                                },
                                (Direction::North, Direction::East),
                            );
                        }
                    },
                    (col_index, '/') => match bottom_start {
                        Some(start) => match incomplete_circuits.remove(&(start, col_index)) {
                            Some(_) => {
                                turns.insert(
                                    Vector {
                                        x: col_index,
                                        y: row_index,
                                    },
                                    (Direction::West, Direction::North),
                                );
                                bottom_start = None;
                            }
                            None => {
                                return Err(From::from(
                                    "Malformed track, circuit bottom without top",
                                ))
                            }
                        },
                        None => {
                            top_start = Some(col_index);
                            turns.insert(
                                Vector {
                                    x: col_index,
                                    y: row_index,
                                },
                                (Direction::East, Direction::South),
                            );
                        }
                    },
                    (col_index, '+') => {
                        intersections.insert(Vector {
                            x: col_index,
                            y: row_index,
                        });
                    }
                    (col_index, '^') => carts.push(Cart {
                        position: Vector {
                            x: col_index,
                            y: row_index,
                        },
                        direction: Direction::North,
                        next_turn: 0,
                    }),
                    (col_index, 'v') => carts.push(Cart {
                        position: Vector {
                            x: col_index,
                            y: row_index,
                        },
                        direction: Direction::South,
                        next_turn: 0,
                    }),
                    (col_index, '>') => carts.push(Cart {
                        position: Vector {
                            x: col_index,
                            y: row_index,
                        },
                        direction: Direction::East,
                        next_turn: 0,
                    }),
                    (col_index, '<') => carts.push(Cart {
                        position: Vector {
                            x: col_index,
                            y: row_index,
                        },
                        direction: Direction::West,
                        next_turn: 0,
                    }),
                    _ => {}
                }
            }
        }

        Ok(Track {
            carts,
            turns,
            intersections,
        })
    }
}

impl Direction {
    fn turn(self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            Turn::Reverse => match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            },
            Turn::Straight => self,
        }
    }
}

impl Cart {
    fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    fn follow_turn(&mut self, turn_arms: (Direction, Direction)) {
        let entering_from = self.direction.turn(Turn::Reverse);

        if entering_from == turn_arms.0 {
            self.direction = turn_arms.1;
        } else if entering_from == turn_arms.1 {
            self.direction = turn_arms.0;
        } else {
            panic!("Cart entered turn from an invalid direction");
        }
    }
}

impl Track {
    fn run_tick(&mut self, find_final_cart: bool) -> Option<Vector> {
        let mut collided_cart_indices = HashSet::new();
        let mut cart_positions: HashMap<Vector, usize> = HashMap::new();
        for (index, cart) in self.carts.iter().enumerate() {
            cart_positions.insert(cart.position, index);
        }

        for (index, cart) in self.carts.iter_mut().enumerate() {
            if collided_cart_indices.contains(&index) {
                continue;
            }

            let Vector { x, y } = cart.position;
            cart_positions.remove(&cart.position);
            cart.position = match cart.direction {
                Direction::North => Vector { x, y: y - 1 },
                Direction::South => Vector { x, y: y + 1 },
                Direction::East => Vector { x: x + 1, y },
                Direction::West => Vector { x: x - 1, y },
            };

            if let Some(turn_arms) = self.turns.get(&cart.position) {
                cart.follow_turn(*turn_arms);
            }

            if self.intersections.contains(&cart.position) {
                cart.turn(INTER_SEQ[cart.next_turn as usize]);
                cart.next_turn = (cart.next_turn + 1) % INTER_SEQ.len() as u8;
            }

            if let Some(colliding_cart) = cart_positions.get(&cart.position) {
                if find_final_cart {
                    collided_cart_indices.insert(index);
                    collided_cart_indices.insert(*colliding_cart);
                    cart_positions.remove(&cart.position);
                    continue;
                } else {
                    return Some(cart.position);
                }
            }

            cart_positions.insert(cart.position, index);
        }

        self.carts = self
            .carts
            .drain(..)
            .enumerate()
            .filter(|(i, _)| !collided_cart_indices.contains(i))
            .map(|(_, cart)| cart)
            .collect();
        self.carts.sort_unstable();
        None
    }

    fn find_first_collision(&mut self) -> Vector {
        let mut collision: Option<Vector> = None;
        while collision.is_none() {
            collision = self.run_tick(false);
        }
        collision.unwrap()
    }

    fn find_last_cart(&mut self) -> &Cart {
        while self.carts.len() != 1 {
            self.run_tick(true);
        }
        &self.carts[0]
    }
}

fn read_track(filename: &str) -> Result<Track> {
    let input = fs::read_to_string(filename)?;
    Ok(input.parse()?)
}

pub fn solve_part1() -> Result<Vector> {
    let mut track = read_track(INPUT)?;
    Ok(track.find_first_collision())
}

pub fn solve_part2() -> Result<Vector> {
    let mut track = read_track(INPUT)?;
    Ok(track.find_last_cart().position)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/13_test.txt";
    fn test_track() -> Track {
        Track {
            carts: vec![
                Cart {
                    position: Vector { x: 2, y: 0 },
                    direction: Direction::East,
                    next_turn: 0,
                },
                Cart {
                    position: Vector { x: 9, y: 3 },
                    direction: Direction::South,
                    next_turn: 0,
                },
            ],
            turns: vec![
                (Vector { x: 0, y: 0 }, (Direction::East, Direction::South)),
                (Vector { x: 4, y: 0 }, (Direction::South, Direction::West)),
                (Vector { x: 4, y: 4 }, (Direction::West, Direction::North)),
                (Vector { x: 0, y: 4 }, (Direction::North, Direction::East)),
                (Vector { x: 7, y: 1 }, (Direction::East, Direction::South)),
                (Vector { x: 12, y: 1 }, (Direction::South, Direction::West)),
                (Vector { x: 12, y: 4 }, (Direction::West, Direction::North)),
                (Vector { x: 7, y: 4 }, (Direction::North, Direction::East)),
                (Vector { x: 2, y: 2 }, (Direction::East, Direction::South)),
                (Vector { x: 9, y: 2 }, (Direction::South, Direction::West)),
                (Vector { x: 9, y: 5 }, (Direction::West, Direction::North)),
                (Vector { x: 2, y: 5 }, (Direction::North, Direction::East)),
            ]
            .iter()
            .cloned()
            .collect(),
            intersections: vec![
                Vector { x: 4, y: 2 },
                Vector { x: 7, y: 2 },
                Vector { x: 2, y: 4 },
                Vector { x: 9, y: 4 },
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    #[test]
    fn reads_track_file() {
        let track = read_track(TEST_INPUT).unwrap();
        assert_eq!(track, test_track());
    }

    #[test]
    fn runs_one_tick() {
        let mut track_after = test_track();
        track_after.carts[0].position.x = 3;
        track_after.carts[1].position.y = 4;
        track_after.carts[1].direction = Direction::East;
        track_after.carts[1].next_turn = 1;
        let mut track = test_track();
        track.run_tick(false);
        assert_eq!(track, track_after);
    }

    #[test]
    fn finds_first_collision() {
        let mut track = test_track();
        assert_eq!(track.find_first_collision(), Vector { x: 7, y: 3 });
    }
}
