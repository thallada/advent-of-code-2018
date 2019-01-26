extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;

use regex::{Captures, Regex};

type Result<T> = result::Result<T, Box<Error>>;

type Instructions = HashMap<char, Vec<char>>;

const INPUT: &str = "inputs/7.txt";
static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn solve_part1() -> Result<String> {
    let mut instructions = read_instructions(INPUT)?;
    Ok(get_step_sequence(&mut instructions))
}

pub fn solve_part2() -> Result<u32> {
    let mut pool = WorkerPool::new(5);
    let mut instructions = read_instructions(INPUT)?;
    Ok(get_parallel_step_sequence_seconds(&mut instructions, &mut pool))
}

fn read_instructions(filename: &str) -> Result<Instructions> {
    let mut instructions: Instructions = HashMap::new();
    lazy_static! {
        static ref INSTRUCTION_REGEX: Regex = Regex::new(
            r"Step (?P<dependency>\w) must be finished before step (?P<step>\w) can begin."
        )
        .unwrap();
    }
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match INSTRUCTION_REGEX.captures(&line?) {
            Some(captures) => {
                let step = get_captured_field(&captures, "step")?;
                let dependency: char = get_captured_field(&captures, "dependency")?;
                instructions.entry(dependency).or_insert_with(Vec::new);
                let dependencies = instructions.entry(step).or_insert_with(Vec::new);
                dependencies.push(dependency);
            }
            None => {
                return Err(From::from(
                    "Malformed instruction line, no fields could be found",
                ))
            }
        };
    }
    Ok(instructions)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<char> {
    match captures.name(field) {
        Some(capture) => match capture.as_str().chars().next() {
            Some(letter) => Ok(letter),
            None => Err(From::from(format!(
                "Malformed instruction line, field {} not a char",
                field
            ))),
        },
        None => Err(From::from(format!(
            "Malformed instruction line, field {} could not be found",
            field
        ))),
    }
}

fn get_step_sequence(instructions: &mut Instructions) -> String {
    let mut sequence = String::new();
    loop {
        let mut available: Vec<char> = instructions
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(step, _)| *step)
            .collect();
        if available.is_empty() {
            break;
        }
        available.sort();
        available.reverse();
        let next = available.pop().unwrap();
        instructions.remove(&next);
        for dependencies in instructions.values_mut() {
            if let Some(index) = dependencies.iter().position(|d| *d == next) {
                dependencies.remove(index);
            }
        }
        sequence.push(next);
    }
    sequence
}

fn get_parallel_step_sequence_seconds(
    mut instructions: &mut Instructions,
    worker_pool: &mut WorkerPool,
) -> u32 {
    let mut sequence = String::new();
    let mut seconds = 0;
    loop {
        worker_pool.run_one_second(&mut instructions, &mut sequence);

        let mut available: Vec<char> = instructions
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(step, _)| *step)
            .collect();

        if available.is_empty() && worker_pool.all_idle() {
            break;
        }

        available.sort();
        available.reverse();

        for mut worker in worker_pool.available() {
            let next = match available.pop() {
                None => break,
                Some(next) => next,
            };
            instructions.remove(&next);
            worker_pool.assign_worker(worker.id, next);
        }
        seconds += 1;
    }
    println!("{}", sequence);
    seconds
}

fn get_seconds_for_step(step_letter: char) -> u8 {
    ALPHABET.iter().position(|&c| c == step_letter).unwrap_or(0) as u8 + 61 as u8
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Worker {
    id: u8,
    status: Status,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Idle,
    Working { step: char, remaining: u8 },
}

#[derive(Debug, PartialEq)]
struct WorkerPool {
    workers: Vec<Worker>,
}

impl WorkerPool {
    fn new(count: u8) -> WorkerPool {
        let mut workers = Vec::new();
        for i in 0..count {
            workers.push(Worker {
                id: i,
                status: Status::Idle,
            })
        }
        WorkerPool { workers }
    }

    fn available(&self) -> Vec<Worker> {
        self.workers
            .iter()
            .filter(|worker| match worker.status {
                Status::Idle => true,
                Status::Working { .. } => false,
            })
            .cloned()
            .collect()
    }

    fn all_idle(&self) -> bool {
        self.workers
            .iter()
            .all(|worker| worker.status == Status::Idle)
    }

    fn run_one_second(&mut self, instructions: &mut Instructions, sequence: &mut String) {
        let new_workers = self
            .workers
            .iter()
            .map(|worker| Worker {
                id: worker.id,
                status: match worker.status {
                    Status::Idle => Status::Idle,
                    Status::Working { step, remaining } => {
                        if remaining == 1 {
                            for dependencies in instructions.values_mut() {
                                if let Some(index) = dependencies.iter().position(|d| *d == step) {
                                    dependencies.remove(index);
                                }
                            }
                            sequence.push(step);
                            Status::Idle
                        } else {
                            Status::Working {
                                step,
                                remaining: remaining - 1,
                            }
                        }
                    }
                },
            })
            .collect();
        self.workers = new_workers;
    }

    fn assign_worker(&mut self, id: u8, step: char) {
        let new_workers = self
            .workers
            .iter()
            .map(|worker| {
                if worker.id == id {
                    Worker {
                        id: worker.id,
                        status: Status::Working {
                            step,
                            remaining: get_seconds_for_step(step),
                        },
                    }
                } else {
                    *worker
                }
            })
            .collect();
        self.workers = new_workers;
    }
}

impl fmt::Display for WorkerPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for worker in &self.workers {
            writeln!(f, "{}", worker)?;
        }
        Ok(())
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            Status::Idle => write!(f, "{}: idle", self.id),
            Status::Working { step, remaining } => {
                write!(f, "{}: {} - {}", self.id, step, remaining)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/7_test.txt";

    fn test_instructions() -> Instructions {
        [
            ('A', vec!['C']),
            ('F', vec!['C']),
            ('C', vec![]),
            ('B', vec!['A']),
            ('D', vec!['A']),
            ('E', vec!['B', 'D', 'F']),
        ]
        .iter()
        .cloned()
        .collect()
    }

    #[test]
    fn reads_instructions_file() {
        assert_eq!(read_instructions(TEST_INPUT).unwrap(), test_instructions());
    }

    #[test]
    fn gets_step_sequence() {
        assert_eq!(get_step_sequence(&mut test_instructions()), "CABDFE");
    }

    #[test]
    fn new_worker_pool() {
        assert_eq!(
            WorkerPool::new(3),
            WorkerPool {
                workers: vec![
                    Worker {
                        id: 0,
                        status: Status::Idle
                    },
                    Worker {
                        id: 1,
                        status: Status::Idle
                    },
                    Worker {
                        id: 2,
                        status: Status::Idle
                    },
                ]
            }
        )
    }

    #[test]
    fn available_workers_in_pool() {
        let pool = WorkerPool {
            workers: vec![
                Worker {
                    id: 0,
                    status: Status::Idle,
                },
                Worker {
                    id: 1,
                    status: Status::Working {
                        step: 'A',
                        remaining: 1,
                    },
                },
                Worker {
                    id: 2,
                    status: Status::Idle,
                },
            ],
        };
        assert_eq!(
            pool.available(),
            vec![
                Worker {
                    id: 0,
                    status: Status::Idle
                },
                Worker {
                    id: 2,
                    status: Status::Idle
                },
            ]
        )
    }

    #[test]
    fn run_workers_one_second() {
        let mut pool = WorkerPool {
            workers: vec![
                Worker {
                    id: 0,
                    status: Status::Idle,
                },
                Worker {
                    id: 1,
                    status: Status::Working {
                        step: 'A',
                        remaining: 2,
                    },
                },
                Worker {
                    id: 2,
                    status: Status::Idle,
                },
            ],
        };
        pool.run_one_second(&mut test_instructions(), &mut String::new());
        assert_eq!(
            pool,
            WorkerPool {
                workers: vec![
                    Worker {
                        id: 0,
                        status: Status::Idle
                    },
                    Worker {
                        id: 1,
                        status: Status::Working {
                            step: 'A',
                            remaining: 1
                        }
                    },
                    Worker {
                        id: 2,
                        status: Status::Idle
                    },
                ]
            }
        )
    }

    #[test]
    fn run_workers_one_second_and_complete_step() {
        let mut pool = WorkerPool {
            workers: vec![Worker {
                id: 0,
                status: Status::Working {
                    step: 'A',
                    remaining: 1,
                },
            }],
        };
        let mut instructions = [('A', vec![])].iter().cloned().collect();
        let mut sequence = "Z".to_string();
        pool.run_one_second(&mut instructions, &mut sequence);
        assert_eq!(
            pool,
            WorkerPool {
                workers: vec![Worker {
                    id: 0,
                    status: Status::Idle
                },]
            }
        );
        assert_eq!(instructions, [('A', vec![])].iter().cloned().collect());
        assert_eq!(sequence, "ZA".to_string());
    }

    #[test]
    fn worker_pool_all_idle() {
        assert_eq!(WorkerPool::new(3).all_idle(), true);
    }

    #[test]
    fn worker_pool_not_all_idle() {
        assert_eq!(
            WorkerPool {
                workers: vec![
                    Worker {
                        id: 0,
                        status: Status::Idle
                    },
                    Worker {
                        id: 1,
                        status: Status::Working {
                            step: 'A',
                            remaining: 1
                        }
                    },
                ],
            }
            .all_idle(),
            false
        );
    }

    #[test]
    fn assign_step_to_worker_in_pool() {
        let mut pool = WorkerPool::new(2);
        pool.assign_worker(0, 'A');
        assert_eq!(
            pool,
            WorkerPool {
                workers: vec![
                    Worker {
                        id: 0,
                        status: Status::Working {
                            step: 'A',
                            remaining: 61,
                        }
                    },
                    Worker {
                        id: 1,
                        status: Status::Idle,
                    }
                ],
            }
        )
    }

    #[test]
    fn gets_seconds_for_step() {
        assert_eq!(get_seconds_for_step('A'), 61);
        assert_eq!(get_seconds_for_step('B'), 62);
        assert_eq!(get_seconds_for_step('Z'), 86);
    }

    #[test]
    fn gets_sequence_with_workers() {
        let mut pool = WorkerPool::new(2);
        let mut instructions = test_instructions();
        assert_eq!(get_parallel_step_sequence_seconds(&mut instructions, &mut pool), 258);
    }
}
