use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day6take2;
mod day7;
mod day8;

fn main() {
    let input = fs::read_to_string("src/day8/input.txt").expect("Cannot read file");
    let input = fs::read_to_string("src/day8/test_input.txt").expect("Cannot read file");

    day8::solution::task1(input);
}

////////////////////////////////////////////////////////////////////////////////
/// Position
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn moved(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Self { x: self.x, y: self.y - 1 },
            Direction::Right => Self { x: self.x + 1, y: self.y },
            Direction::Down => Self { x: self.x, y: self.y + 1 },
            Direction::Left => Self { x: self.x - 1, y: self.y },
        }
    }

    fn delta(&self, position: &Position) -> Position {
        Position {
            x: position.x.abs_diff(self.x),
            y: position.y.abs_diff(self.y),
        }
    }

    fn concat(&self, position: &Position) -> Position {
        Position { x: position.x + self.x, y: position.y + self.y }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Direction
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        use Direction::*;
        [Right, Down, Left, Up][(*self as usize + 1) % 4]
    }
    fn turn_left(&self) -> Direction {
        use Direction::*;
        [Right, Down, Left, Up][(*self as usize - 1) % 4]
    }
}
