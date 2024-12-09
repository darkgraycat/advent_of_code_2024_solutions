use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = fs::read_to_string("src/day6/input.txt").expect("Cannot read file");
    day6::solution::task1(input);
}
