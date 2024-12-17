use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day6take2;
mod day7;

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").expect("Cannot read file");
    let input = fs::read_to_string("src/day7/test_input.txt").expect("Cannot read file");

    day7::solution::task2(input);

    // 15404463818899 - is too low
}
