mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Day 1:");
    println!("{}", day1::solve_part1().unwrap());
    println!("{}", day1::solve_part2().unwrap().unwrap());
    println!("Day 2:");
    println!("{}", day2::solve_part1().unwrap());
    println!("{}", day2::solve_part2().unwrap().unwrap());
    println!("Day 3:");
    println!("{}", day3::solve_part1().unwrap());
    println!("{}", day3::solve_part2().unwrap().unwrap());
    println!("Day 4:");
    println!("{}", day4::solve_part1().unwrap());
}
