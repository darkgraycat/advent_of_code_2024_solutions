use std::fs;

mod day1;
mod day2;

fn main() {
    let input = fs::read_to_string("src/day2/test_input.txt").expect("Cannot read file");
    day2::solution::task1(input);
}
