use std::fs;

mod day1;

fn main() {
    let input = fs::read_to_string("src/day1/input_1.txt").expect("Cannot read file");
    day1::task1::run(input);
}
