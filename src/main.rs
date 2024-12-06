use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    let input = fs::read_to_string("src/day3/input.txt").expect("Cannot read file");
    day3::solution::task1(input);
}
