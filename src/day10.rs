extern crate regex;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::result;
use std::str::FromStr;

use regex::Regex;

type Result<T> = result::Result<T, Box<Error>>;

const INPUT: &str = "inputs/10.txt";

#[derive(Debug, PartialEq, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Point {
    position: Vector,
    velocity: Vector,
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Point> {
        lazy_static! {
            static ref RE: Regex = Regex::new(concat!(
                r"position=<(?P<position_x>(\s|-)?\d+), (?P<position_y>(\s|-)?\d+)> ",
                r"velocity=<(?P<velocity_x>(\s|-)?\d+), (?P<velocity_y>(\s|-)?\d+)>"
            ))
            .unwrap();
        }

        let captures = match RE.captures(s) {
            None => {
                return Err(From::from("Malformed points, no fields could be found"));
            }
            Some(captures) => captures,
        };

        Ok(Point {
            position: Vector {
                x: captures["position_x"].trim_start().parse()?,
                y: captures["position_y"].trim_start().parse()?,
            },
            velocity: Vector {
                x: captures["velocity_x"].trim_start().parse()?,
                y: captures["velocity_y"].trim_start().parse()?,
            },
        })
    }
}

#[derive(Debug, PartialEq)]
struct Sky {
    points: Vec<Point>,
}

impl FromStr for Sky {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Sky> {
        Ok(Sky {
            points: s
                .trim_end()
                .split('\n')
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}

impl Sky {
    fn point_spread(&self) -> (Vector, Vector) {
        let mut min = self.points[0].position.clone();
        let mut max = self.points[0].position.clone();

        for point in self.points.iter() {
            if point.position.x < min.x {
                min.x = point.position.x;
            }
            if point.position.y < min.y {
                min.y = point.position.y;
            }
            if point.position.x > max.x {
                max.x = point.position.x;
            }
            if point.position.y > max.y {
                max.y = point.position.y;
            }
        }

        (min, max)
    }

    fn move_points(&mut self, seconds: i32) {
        for point in self.points.iter_mut() {
            point.position.x += point.velocity.x * seconds;
            point.position.y += point.velocity.y * seconds;
        }
    }
}

impl fmt::Display for Sky {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min, max) = self.point_spread();
        let points_set: HashSet<(i32, i32)> = self
            .points
            .iter()
            .map(|point| (point.position.x, point.position.y))
            .collect();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if points_set.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn solve_parts() -> Result<(String, u32)> {
    let mut sky = read_points_file(INPUT)?;
    let (min, max) = sky.point_spread();
    let mut min_spread = Vector {
        x: (max.x - min.x).abs(),
        y: (max.y - min.y).abs(),
    };
    let mut seconds = 0;

    loop {
        sky.move_points(1);
        let (min, max) = sky.point_spread();
        let spread_x = (max.x - min.x).abs();
        let spread_y = (max.y - min.y).abs();

        if spread_x > min_spread.x && spread_y > min_spread.y {
            sky.move_points(-1);
            return Ok((format!("{}", &sky), seconds));
        }

        if spread_x < min_spread.x {
            min_spread.x = spread_x
        }
        if spread_y < min_spread.y {
            min_spread.y = spread_y
        }
        seconds += 1;
    }
}

fn read_points_file(filename: &str) -> Result<Sky> {
    let points = fs::read_to_string(filename)?;
    Ok(points.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_ONE: &str = "inputs/10_test_one.txt";
    const TEST_INPUT: &str = "inputs/10_test.txt";
    const TEST_POINT_1: Point = Point {
        position: Vector { x: 0, y: 1 },
        velocity: Vector { x: 0, y: 0 },
    };
    const TEST_POINT_2: Point = Point {
        position: Vector { x: 0, y: 0 },
        velocity: Vector { x: 0, y: 0 },
    };
    fn test_sky() -> Sky {
        Sky {
            points: vec![TEST_POINT_1, TEST_POINT_2],
        }
    }

    #[test]
    fn parses_string_to_point() {
        let point = "position=< 0,  1> velocity=< 0,  0>";
        assert_eq!(point.parse::<Point>().unwrap(), TEST_POINT_1);
    }

    #[test]
    fn reads_points_file() {
        assert_eq!(
            read_points_file(TEST_INPUT_ONE).unwrap(),
            Sky {
                points: vec![TEST_POINT_1]
            }
        );
    }

    #[test]
    fn displays_sky_with_one_point() {
        assert_eq!(format!("{}", test_sky()), "#\n#\n");
    }

    #[test]
    fn displays_sky() {
        let sky = read_points_file(TEST_INPUT).unwrap();
        print!("{}", &sky);
        assert_eq!(
            format!("{}", sky),
            concat!(
                "........#.............\n",
                "................#.....\n",
                ".........#.#..#.......\n",
                "......................\n",
                "#..........#.#.......#\n",
                "...............#......\n",
                "....#.................\n",
                "..#.#....#............\n",
                ".......#..............\n",
                "......#...............\n",
                "...#...#.#...#........\n",
                "....#..#..#.........#.\n",
                ".......#..............\n",
                "...........#..#.......\n",
                "#...........#.........\n",
                "...#.......#..........\n",
            )
        );
    }

    #[test]
    fn displays_message_in_sky() {
        let mut sky = read_points_file(TEST_INPUT).unwrap();
        sky.move_points(3);
        print!("{}", &sky);
        assert_eq!(
            format!("{}", sky),
            concat!(
                "#...#..###\n",
                "#...#...#.\n",
                "#...#...#.\n",
                "#####...#.\n",
                "#...#...#.\n",
                "#...#...#.\n",
                "#...#...#.\n",
                "#...#..###\n",
            )
        );
    }
}
