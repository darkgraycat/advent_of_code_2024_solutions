use std::{
    collections::HashSet,
    ops::Range,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn get_next(&self) -> Position {
        self.position.move_to(self.direction)
    }
    fn turn(&mut self) {
        self.direction = self.direction.turn_right();
    }
    fn forward(&mut self) {
        self.position = self.get_next();
    }
}

#[derive(Debug, Clone)]
struct State {
    guard: Guard,
    blocks: HashSet<Position>,
    visited: HashSet<(Position, Direction)>,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl State {
    fn is_looping(&self) -> bool {
        self.visited
            .contains(&(self.guard.position, self.guard.direction))
    }

    fn make_step(&mut self) -> Option<()> { 
        let next_position = self.get_next_position()?;

        if self.blocks.contains(&next_position) {
            self.guard.turn();
        } else {
            self.guard.forward();
        }

        Some(())
    }

    fn get_next_position(&self) -> Option<Position> {
        let next = self.guard.get_next();
        (self.x_range.contains(&next.x) && self.y_range.contains(&next.y)).then_some(next)
    }

    fn with_block(&self, position: Position) -> Self {
        let mut cloned = self.clone();
        cloned.blocks.insert(position);
        cloned
    }
}

impl TryFrom<String> for State {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let blocks: HashSet<Position> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch == '#')
                    .map(move |(x, _)| Position::new(x, y))
            })
            .collect();

        let guard: Guard = value
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars().position(|c| c == '^').map(|x| Guard {
                    position: Position::new(x, y),
                    direction: Direction::Up,
                })
            })
            .ok_or("Guard not found")?;

        Ok(State {
            guard,
            blocks,
            visited: HashSet::new(),
            x_range: 0..value.lines().last().unwrap().len(),
            y_range: 0..value.lines().count(),
        })
    }
}

pub fn task2(input: String) {
    let state = State::try_from(input).expect("Pew pew");
    println!("Initial state {:?}", state);
}
