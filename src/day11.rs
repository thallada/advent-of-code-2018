use std::error::Error;
use std::fmt;
use std::fs;
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/11.txt";
const GRID_SIZE: usize = 300;

#[derive(Clone)]
struct Cells([[Option<i8>; GRID_SIZE]; GRID_SIZE]);

impl fmt::Display for Cells {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                match self.0[x][y] {
                    Some(power) => write!(f, "{}", power)?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Cells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cells {{\n{}\n}}", self)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Grid {
    serial_number: usize,
    cells: Cells,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Subsection {
    pub coord: Coordinate,
    pub size: usize,
}

impl Grid {
    fn new(serial_number: usize) -> Grid {
        Grid {
            serial_number,
            cells: Cells([[None; GRID_SIZE]; GRID_SIZE]),
        }
    }

    fn power_at_cell(&self, coord: &Coordinate) -> i8 {
        let rack_id = coord.x + 10;
        let mut power_level = rack_id * coord.y;
        power_level += self.serial_number;
        power_level *= rack_id;
        power_level = power_level / 100 % 10;
        power_level as i8 - 5
    }

    fn power_of_subsection(&mut self, subsection: &Subsection) -> i32 {
        let mut power_level: i32 = 0;
        let Subsection { coord, size } = subsection;
        for x in coord.x..coord.x + size {
            for y in coord.y..coord.y + size {
                power_level += i32::from(match self.cells.0[x][y] {
                    Some(power) => power,
                    None => {
                        let power = self.power_at_cell(&Coordinate { x, y });
                        self.cells.0[x][y] = Some(power);
                        power
                    }
                });
            }
        }
        power_level
    }

    fn highest_power_subsection(&mut self, size: usize) -> (Subsection, i32) {
        let mut highest_power_subsection = Subsection {
            coord: Coordinate { x: 0, y: 0 },
            size,
        };
        let mut highest_power_level = self.power_of_subsection(&highest_power_subsection);
        for x in 0..GRID_SIZE - size {
            for y in 0..GRID_SIZE - size {
                let subsection = Subsection {
                    coord: Coordinate { x, y },
                    size,
                };
                if subsection == highest_power_subsection {
                    continue;
                };
                let power = self.power_of_subsection(&subsection);
                if power > highest_power_level {
                    highest_power_subsection = subsection;
                    highest_power_level = power;
                }
            }
        }
        (highest_power_subsection, highest_power_level)
    }
}

fn read_serial_number_file(filename: &str) -> Result<usize> {
    let serial_number = fs::read_to_string(filename)?;
    Ok(serial_number.trim().parse()?)
}

pub fn solve_part1() -> Result<Subsection> {
    let serial_number = read_serial_number_file(INPUT)?;
    let mut grid = Grid::new(serial_number);
    Ok(grid.highest_power_subsection(3).0)
}

pub fn solve_part2() -> Result<Subsection> {
    let serial_number = read_serial_number_file(INPUT)?;
    let mut grid = Grid::new(serial_number);
    let (mut highest_power_subsection, mut highest_power_level) = grid.highest_power_subsection(1);

    for size in 2..=GRID_SIZE {
        let (subsection, power) = grid.highest_power_subsection(size);
        if power > highest_power_level {
            highest_power_subsection = subsection;
            highest_power_level = power;
        }
    }
    Ok(highest_power_subsection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_empty_grid() {
        let grid = Grid::new(18);
        assert_eq!(grid.serial_number, 18);
    }

    #[test]
    fn returns_power_at_cell() {
        let grid = Grid::new(8);
        assert_eq!(grid.power_at_cell(&Coordinate { x: 3, y: 5 }), 4);
        let grid = Grid::new(57);
        assert_eq!(grid.power_at_cell(&Coordinate { x: 122, y: 79 }), -5);
        let grid = Grid::new(39);
        assert_eq!(grid.power_at_cell(&Coordinate { x: 217, y: 196 }), 0);
        let grid = Grid::new(71);
        assert_eq!(grid.power_at_cell(&Coordinate { x: 101, y: 153 }), 4);
    }

    #[test]
    fn returns_power_of_subsection() {
        let mut grid = Grid::new(18);
        assert_eq!(
            grid.power_of_subsection(&Subsection {
                coord: Coordinate { x: 33, y: 45 },
                size: 3
            }),
            29
        );
    }

    #[test]
    fn returns_highest_power_subsection() {
        let mut grid = Grid::new(18);
        assert_eq!(
            grid.highest_power_subsection(3),
            (
                Subsection {
                    coord: Coordinate { x: 33, y: 45 },
                    size: 3
                },
                29
            )
        );
    }
}
