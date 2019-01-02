extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::{HashMap, HashSet};

use regex::{Regex, Captures};

static ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
];
const INPUT: &str = "inputs/6.txt";

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Coordinate {
    x: u32,
    y: u32,
    letter: char,
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

#[derive(Debug, PartialEq)]
struct Grid {
    points: Vec<GridPoint>,
    boundary_coord: Coordinate,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.letter)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n-----")?;
        for (index, point) in self.points.iter().enumerate() {
            if index as u32 % (self.boundary_coord.x + 1) == 0 {
                write!(f, "\n")?;
            }
            match point {
                GridPoint::Unfilled { x: _, y: _ } => { write!(f, "-")?; },
                GridPoint::Tied { x: _, y: _, closest_dist: _} => {
                    write!(f, ".")?;
                },
                GridPoint::Filled { x, y, closest_coord, closest_dist: _ } => {
                    if *x == closest_coord.x && *y == closest_coord.y {
                        write!(f, "#")?;
                    } else {
                        write!(f, "{}", closest_coord)?;
                    }
                },
            }
        }
        write!(f, "\n-----")?;
        Ok(())
    }
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

pub fn solve_part1() -> Result<u32, Box<Error>> {
    let coords = read_coordinates(INPUT)?;
    let boundary_coord = get_boundary_coordinate(&coords);
    let mut grid = create_grid(boundary_coord);
    fill_grid(&mut grid, &coords).unwrap();
    println!("{}", grid);
    Ok(find_largest_coord_area(grid))
}

pub fn solve_part2() -> Result<u32, Box<Error>> {
    let coords = read_coordinates(INPUT)?;
    let boundary_coord = get_boundary_coordinate(&coords);
    let grid = create_grid(boundary_coord);
    Ok(region_closest_to_coordinates_size(grid, coords))
}

fn read_coordinates(filename: &str) -> Result<Vec<Coordinate>, Box<Error>> {
    let mut records: Vec<Coordinate> = Vec::new();
    lazy_static! {
        static ref COORDINATE_REGEX: Regex = Regex::new(
            r"(?P<x>\d+), (?P<y>\d+)").unwrap();
    }
    let file = File::open(filename)?;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        match COORDINATE_REGEX.captures(&line?) {
            Some(captures) => {
                records.push(Coordinate {
                    x: get_captured_field(&captures, "x")?.parse()?,
                    y: get_captured_field(&captures, "y")?.parse()?,
                    letter: ALPHABET[index],
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

fn get_boundary_coordinate(coords: &Vec<Coordinate>) -> Coordinate {
    let mut boundary_coord = Coordinate { x: 0, y: 0, letter: '+' };
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

fn create_grid(boundary_coord: Coordinate) -> Grid {
    let mut points = Vec::new();
    for y in 0..boundary_coord.y + 1 {
        for x in 0..boundary_coord.x + 1 {
            points.push(GridPoint::Unfilled { x, y });
        }
    }
    Grid { points, boundary_coord }
}

fn fill_grid<'a>(
    grid: &'a mut Grid,
    coords: &'a Vec<Coordinate>,
) -> Result<&'a mut Grid, Box<Error>> {
    for coord in coords {
        let start_index = (coord.x * (grid.boundary_coord.y + 1)) + coord.y;
        fill_grid_with_coordinate(
            grid,
            start_index,
            *coord,
        )?;
    }
    Ok(grid)
}

fn fill_grid_with_coordinate(
    grid: &mut Grid,
    index: u32,
    coord: Coordinate,
) -> Result<&mut Grid, Box<Error>> {
    let mut visited_indices = HashSet::new();
    for point in &mut grid.points {
        visited_indices.insert(index);
        match *point {
            GridPoint::Unfilled { x, y } => {
                *point = GridPoint::Filled {
                    x: x,
                    y: y,
                    closest_coord: coord,
                    closest_dist: manhattan_dist(coord.x, coord.y, x, y),
                };
            },
            GridPoint::Tied { x, y, closest_dist } => {
                let dist = manhattan_dist(coord.x, coord.y, x, y);
                if dist < closest_dist {
                    *point = GridPoint::Filled {
                        x: x,
                        y: y,
                        closest_coord: coord,
                        closest_dist: dist,
                    };
                }
            },
            GridPoint::Filled { x, y, closest_coord, closest_dist } => {
                let dist = manhattan_dist(coord.x, coord.y, x, y);
                if dist < closest_dist {
                    *point = GridPoint::Filled {
                        x: x,
                        y: y,
                        closest_coord: coord,
                        closest_dist: dist,
                    };
                } else if dist == closest_dist && closest_coord != coord {
                    *point = GridPoint::Tied {
                        x: x,
                        y: y,
                        closest_dist: dist,
                    };
                }
            },
        }
    }
    Ok(grid)
}

fn manhattan_dist(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    ((x2 as i32 - x1 as i32).abs() + (y2 as i32 - y1 as i32).abs()) as u32
}

fn find_largest_coord_area(
    grid: Grid,
) -> u32 {
    let mut point_count = HashMap::new();
    let mut infinite_coords = HashSet::new();
    for point in grid.points.iter() {
        match point {
            GridPoint::Filled { x, y, closest_coord: coord, closest_dist: _ } => {
                if *x == 0 || *x == grid.boundary_coord.x ||
                    *y == 0 || *y == grid.boundary_coord.y {
                    point_count.remove(coord);
                    infinite_coords.insert(coord);
                    continue;
                }
                if !infinite_coords.contains(coord) {
                    let count = point_count.entry(coord).or_insert(0);
                    *count += 1;
                }
            },
            _ => ()
        }
    }
    *point_count.values().max().unwrap_or(&0)
}

fn region_closest_to_coordinates_size(grid: Grid, coords: Vec<Coordinate>) -> u32 {
    let mut points_in_region = 0;
    for point in grid.points.iter() {
        match point {
            GridPoint::Filled { x, y, closest_coord: _, closest_dist: _ } |
                GridPoint::Tied { x, y, closest_dist: _ } |
                GridPoint::Unfilled { x, y } => {
                let mut sum = 0;
                for coord in coords.iter() {
                    sum += manhattan_dist(coord.x, coord.y, *x, *y);
                }
                if sum < 10000 {
                    points_in_region += 1;
                }
            }
        }
    }
    points_in_region
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
                letter: 'a',
            },
            Coordinate {
                x: 1,
                y: 6,
                letter: 'b',
            },
            Coordinate {
                x: 8,
                y: 3,
                letter: 'c',
            },
            Coordinate {
                x: 3,
                y: 4,
                letter: 'd',
            },
            Coordinate {
                x: 5,
                y: 5,
                letter: 'e',
            },
            Coordinate {
                x: 8,
                y: 9,
                letter: 'f',
            },
        ]);
    }

    #[test]
    fn gets_boundary_coordinate() {
        assert_eq!(get_boundary_coordinate(&vec![
            Coordinate {
                x: 1,
                y: 1,
                letter: 'a',
            },
            Coordinate {
                x: 5,
                y: 5,
                letter: 'b',
            },
            Coordinate {
                x: 2,
                y: 7,
                letter: 'c',
            }
        ]),
            Coordinate {
                x: 5,
                y: 7,
                letter: '+',
            }
        )
    }

    #[test]
    fn creates_grid() {
        let boundary_coord = Coordinate { x: 1, y: 1, letter: '+' };
        assert_eq!(
            create_grid(boundary_coord),
            Grid {
                points: vec![
                    GridPoint::Unfilled {
                        x: 0,
                        y: 0,
                    },
                    GridPoint::Unfilled {
                        x: 1,
                        y: 0,
                    },
                    GridPoint::Unfilled {
                        x: 0,
                        y: 1,
                    },
                    GridPoint::Unfilled {
                        x: 1,
                        y: 1,
                    },
                ],
                boundary_coord,
            })
    }

    #[test]
    fn calculates_manhattan_dist() {
        assert_eq!(manhattan_dist(0, 0, 2, 1), 3);
        assert_eq!(manhattan_dist(0, 0, 0, 0), 0);
        assert_eq!(manhattan_dist(2, 1, 0, 0), 3);
    }

    #[test]
    fn fills_grid_with_one_coord() {
        let boundary_coord = Coordinate { x: 1, y: 1, letter: '+' };
        let mut grid = create_grid(boundary_coord);
        let coord = Coordinate { x: 0, y: 0, letter: 'a' };
        assert_eq!(
            fill_grid(&mut grid, &vec![coord]).unwrap(),
            &mut Grid {
                points: vec![
                    GridPoint::Filled {
                        x: 0,
                        y: 0,
                        closest_coord: coord,
                        closest_dist: 0,
                    },
                    GridPoint::Filled {
                        x: 1,
                        y: 0,
                        closest_coord: coord,
                        closest_dist: 1,
                    },
                    GridPoint::Filled {
                        x: 0,
                        y: 1,
                        closest_coord: coord,
                        closest_dist: 1,
                    },
                    GridPoint::Filled {
                        x: 1,
                        y: 1,
                        closest_coord: coord,
                        closest_dist: 2,
                    },
                ],
                boundary_coord
            }
        );
    }

    #[test]
    fn fills_grid_with_two_coords() {
        let boundary_coord = Coordinate { x: 1, y: 1, letter: '+' };
        let mut grid = create_grid(boundary_coord);
        let coord_a = Coordinate { x: 0, y: 0, letter: 'a' };
        let coord_b = Coordinate { x: 1, y: 1, letter: 'b' };
        assert_eq!(
            fill_grid(&mut grid, &vec![coord_a, coord_b]).unwrap(),
            &mut Grid {
                points: vec![
                    GridPoint::Filled {
                        x: 0,
                        y: 0,
                        closest_coord: coord_a,
                        closest_dist: 0,
                    },
                    GridPoint::Tied {
                        x: 1,
                        y: 0,
                        closest_dist: 1,
                    },
                    GridPoint::Tied {
                        x: 0,
                        y: 1,
                        closest_dist: 1,
                    },
                    GridPoint::Filled {
                        x: 1,
                        y: 1,
                        closest_coord: coord_b,
                        closest_dist: 0,
                    },
                ],
                boundary_coord
            }
        );
    }

    #[test]
    fn finds_largest_coord_area() {
        let boundary_coord = Coordinate { x: 2, y: 2, letter: '+' };
        let mut grid = create_grid(boundary_coord);
        let coords = vec![
            Coordinate { x: 0, y: 0, letter: 'a' },
            Coordinate { x: 2, y: 2, letter: 'b' },
            Coordinate { x: 1, y: 1, letter: 'c' },
        ];
        fill_grid(&mut grid, &coords).unwrap();
        assert_eq!(
            find_largest_coord_area(grid),
            1
        );
    }
}
