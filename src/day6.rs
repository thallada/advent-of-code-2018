extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fmt, mem};
use std::collections::HashMap;

use regex::{Regex, Captures};

const INPUT: &str = "inputs/6.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
enum GridPoint {
    Unfilled {
        x: u32,
        y: u32,
    },
    Tied {
        x: u32,
        y: u32,
        closest_dist: u32,
    },
    Filled {
        x: u32,
        y: u32,
        closest_coord: Coordinate,
        closest_dist: u32,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct MalformedCoordinate {
    details: String
}

impl MalformedCoordinate {
    fn new(msg: &str) -> MalformedCoordinate {
        MalformedCoordinate{ details: msg.to_string() }
    }
}

impl fmt::Display for MalformedCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MalformedCoordinate {
    fn description(&self) -> &str {
        &self.details
    }
}

fn read_coordinates(filename: &str) -> Result<Vec<Coordinate>, Box<Error>> {
    let mut records: Vec<Coordinate> = Vec::new();
    lazy_static! {
        static ref COORDINATE_REGEX: Regex = Regex::new(
            r"(?P<x>\d+), (?P<y>\d+)").unwrap();
    }
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        match COORDINATE_REGEX.captures(&line?) {
            Some(captures) => {
                records.push(Coordinate {
                    x: get_captured_field(&captures, "x")?.parse()?,
                    y: get_captured_field(&captures, "y")?.parse()?,
                });
            },
            None => return Err(Box::new(MalformedCoordinate {
                details: "Malformed coordinate line, no fields could be found".to_string()
            })),
        };
    }
    Ok(records)
}

fn get_captured_field(captures: &Captures, field: &str) -> Result<String, Box<Error>> {
    match captures.name(field) {
        Some(capture) => Ok(String::from(capture.as_str())),
        None => return Err(Box::new(MalformedCoordinate {
            details: format!("Malformed coordinate line, field {} could not be found", field)
        }))
    }
}

fn get_boundary_coordinate(coords: Vec<Coordinate>) -> Coordinate {
    let mut boundary_coord = Coordinate { x: 0, y: 0 };
    for coord in coords {
        if coord.x > boundary_coord.x {
            boundary_coord.x = coord.x;
        }
        if coord.y > boundary_coord.y {
            boundary_coord.y = coord.y;
        }
    }
    boundary_coord
}

fn create_grid(boundary_coord: Coordinate) -> Vec<GridPoint> {
    let mut grid = Vec::new();
    for x in 0..boundary_coord.x + 1 {
        for y in 0..boundary_coord.y + 1 {
            grid.push(GridPoint::Unfilled { x, y });
        }
    }
    grid
}

fn fill_grid(
    grid: &mut Vec<GridPoint>,
    coords: Vec<Coordinate>,
    boundary_coord: Coordinate,
) -> Result<&mut Vec<GridPoint>, Box<Error>> {
    for coord in coords {
        let start_index = (coord.x * (boundary_coord.y + 1)) + coord.y;
        fill_grid_with_coordinate(grid, start_index, coord, boundary_coord, 0)?;
    }
    Ok(grid)
}

fn fill_grid_with_coordinate(
    grid: &mut Vec<GridPoint>,
    index: u32,
    coord: Coordinate,
    boundary_coord: Coordinate,
    iterations: u32,
) -> Result<&mut Vec<GridPoint>, Box<Error>> {
    if iterations == 10 { return Ok(grid); }
    println!("index: {}", index);
    println!("grid: {:?}", grid);
    let point = &mut grid.get_mut(index as usize).unwrap();
    match point {
        GridPoint::Unfilled { x, y } => {
            grid[index as usize] = GridPoint::Filled {
                x: *x,
                y: *y,
                closest_coord: coord,
                closest_dist: manhattan_dist(coord.x, coord.y, *x, *y),
            };
        },
        GridPoint::Tied { x, y, closest_dist } |
            GridPoint::Filled { x, y, closest_coord: _, closest_dist } => {
            let dist = manhattan_dist(coord.x, coord.y, *x, *y);
            if dist < *closest_dist {
                grid[index as usize] = GridPoint::Filled {
                    x: *x,
                    y: *y,
                    closest_coord: coord,
                    closest_dist: dist,
                };
            } else if dist == *closest_dist {
                grid[index as usize] = GridPoint::Tied {
                    x: *x,
                    y: *y,
                    closest_dist: dist,
                };
            }
        },
    }
    let row_index = index / (boundary_coord.y + 1);
    let col_index = index % (boundary_coord.y + 1);
    println!("row_index: {}", row_index);
    println!("col_index: {}", col_index);
    // South
    if col_index < boundary_coord.y {
        println!("south: {}", index + 1);
        fill_grid_with_coordinate(grid, index + 1, coord, boundary_coord, iterations + 1)?;
    }
    // North
    if col_index > 0 {
        println!("north: {}", index - 1);
        fill_grid_with_coordinate(grid, index - 1, coord, boundary_coord, iterations + 1)?;
    }
    // East
    if row_index < boundary_coord.x {
        println!("east: {}", index + (boundary_coord.y + 1));
        fill_grid_with_coordinate(grid, index + (boundary_coord.y + 1), coord, boundary_coord, iterations + 1)?;
    }
    // West
    if row_index > 0 {
        println!("west: {}", index - (boundary_coord.y + 1));
        fill_grid_with_coordinate(grid, index - (boundary_coord.y + 1), coord, boundary_coord, iterations + 1)?;
    }
    println!("returning grid");
    Ok(grid)
}

fn manhattan_dist(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    ((x2 as i32 - x1 as i32) + (y2 as i32 - y1 as i32)).abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "inputs/6_test.txt";

    #[test]
    fn read_coordinates_file() {
        assert_eq!(read_coordinates(TEST_INPUT).unwrap(), vec![
            Coordinate {
                x: 1,
                y: 1,
            },
            Coordinate {
                x: 1,
                y: 6,
            },
            Coordinate {
                x: 8,
                y: 3,
            },
            Coordinate {
                x: 3,
                y: 4,
            },
            Coordinate {
                x: 5,
                y: 5,
            },
            Coordinate {
                x: 8,
                y: 9,
            },
        ]);
    }

    #[test]
    fn gets_boundary_coordinate() {
        assert_eq!(get_boundary_coordinate(vec![
            Coordinate {
                x: 1,
                y: 1,
            },
            Coordinate {
                x: 5,
                y: 5,
            },
            Coordinate {
                x: 2,
                y: 7,
            }
        ]),
            Coordinate {
                x: 5,
                y: 7,
            }
        )
    }

    #[test]
    fn creates_grid() {
        assert_eq!(
            create_grid(Coordinate { x: 1, y: 1 }),
            vec![
                GridPoint::Unfilled {
                    x: 0,
                    y: 0,
                },
                GridPoint::Unfilled {
                    x: 0,
                    y: 1,
                },
                GridPoint::Unfilled {
                    x: 1,
                    y: 0,
                },
                GridPoint::Unfilled {
                    x: 1,
                    y: 1,
                },
            ])
    }

    #[test]
    fn calculates_manhattan_dist() {
        assert_eq!(manhattan_dist(0, 0, 2, 1), 3);
        assert_eq!(manhattan_dist(0, 0, 0, 0), 0);
        assert_eq!(manhattan_dist(2, 1, 0, 0), 3);
    }

    #[test]
    fn fills_grid_with_one_coord() {
        let boundary_coord = Coordinate { x: 1, y: 1 };
        let mut grid = create_grid(boundary_coord);
        let coord = Coordinate { x: 0, y: 0 };
        assert_eq!(
            fill_grid(&mut grid, vec![coord], boundary_coord).unwrap(),
            &mut vec![
                GridPoint::Filled {
                    x: 0,
                    y: 0,
                    closest_coord: coord,
                    closest_dist: 0,
                },
                GridPoint::Filled {
                    x: 0,
                    y: 1,
                    closest_coord: coord,
                    closest_dist: 1,
                },
                GridPoint::Filled {
                    x: 1,
                    y: 0,
                    closest_coord: coord,
                    closest_dist: 1,
                },
                GridPoint::Filled {
                    x: 1,
                    y: 1,
                    closest_coord: coord,
                    closest_dist: 2,
                },
            ]
        );
    }

    #[test]
    fn fills_grid_with_two_coords() {
        let boundary_coord = Coordinate { x: 1, y: 1 };
        let mut grid = create_grid(boundary_coord);
        let coord_a = Coordinate { x: 0, y: 0 };
        let coord_b = Coordinate { x: 1, y: 1 };
        assert_eq!(
            fill_grid(&mut grid, vec![coord_a, coord_b], boundary_coord).unwrap(),
            &mut vec![
                GridPoint::Filled {
                    x: 0,
                    y: 0,
                    closest_coord: coord_a,
                    closest_dist: 0,
                },
                GridPoint::Tied {
                    x: 0,
                    y: 1,
                    closest_dist: 1,
                },
                GridPoint::Tied {
                    x: 1,
                    y: 0,
                    closest_dist: 1,
                },
                GridPoint::Filled {
                    x: 1,
                    y: 1,
                    closest_coord: coord_b,
                    closest_dist: 2,
                },
            ]
        );
    }
}
