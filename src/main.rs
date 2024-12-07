use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = fs::read_to_string("src/day5/input.txt").expect("Cannot read file");
    day5::solution::task2(input);
}
